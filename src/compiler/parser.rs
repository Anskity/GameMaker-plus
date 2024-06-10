use crate::compiler::ast::Node;
use crate::compiler::tokenizer::{Token, TokenType};

struct ParseMessage(usize, Node);

pub fn parse(mut tokens: Vec<Token>) -> Node {
    let mut statements: Vec<Node> = Vec::new();

    while tokens.first().unwrap().kind != TokenType::EOF {
        let parse_message = parse_statement(&tokens);
        let ParseMessage(delete_amount, result) = parse_message.unwrap();

        tokens.drain(0..delete_amount);

        if result != Node::Ignore {
            statements.push(result);
        }
    }

    let boxxer = |stmt: &Node| Box::new(stmt.clone());

    let boxed_statements: Vec<Box<Node>> = statements.iter().map(boxxer).collect();

    Node::Program(boxed_statements)
}

fn parse_statement(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    let first_token = tokens.first().unwrap();
    match first_token.kind {
        TokenType::Let | TokenType::Const => parse_variable_declaration(tokens),
        TokenType::Function => parse_function_declaration(tokens),
        TokenType::Semilicon => Ok(ParseMessage(1, Node::Ignore)),
        TokenType::Identifier => parse_expression(tokens),
        TokenType::Return => parse_return(tokens),
        _ => Err(format!(
            "Unexpected token while parsing statement: '{:?}' | '{}'",
            first_token.kind, first_token.lex
        )),
    }
}

fn parse_expression(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    if tokens.len() == 1 {
        return parse_primary(&tokens[0]);
    }

    if tokens.first().unwrap().kind == TokenType::Identifier
        && tokens.get(1).unwrap().kind == TokenType::OpenParenthesis
    {
        let mut close_index = Option::<usize>::None;
        let mut parenthesis_seen = 0;

        for (i, tk) in tokens.iter().enumerate() {
            if tk.kind == TokenType::OpenParenthesis {
                parenthesis_seen += 1;
            } else if tk.kind == TokenType::CloseParenthesis {
                parenthesis_seen -= 1;
                if parenthesis_seen == 0 {
                    close_index = Some(i);
                }
            }
        }

        if close_index == None {
            return Err("Couldn't find enough close parenthesis".to_string());
        }

        return parse_function_call(&tokens[0..=close_index.unwrap()].to_vec());
    }

    parse_additive(tokens)
}

fn parse_binary_expr(left: &Node, operator: &Node, right: &Node) -> Result<ParseMessage, String> {
    use crate::enum_utils::enum_weak_equals;

    if !enum_weak_equals(operator, &Node::BinaryOperator("+".to_string())) {
        return Err(format!("Given operator node isn't valid: {:?}", operator));
    }

    let binary_expr = Node::BinaryExpression(
        Box::new(left.to_owned()),
        Box::new(operator.to_owned()),
        Box::new(right.to_owned()),
    );

    Ok(ParseMessage(3, binary_expr))
}

fn parse_primary(token: &Token) -> Result<ParseMessage, String> {
    let expr = match token.kind {
        TokenType::NumericListeral => Node::NumericLiteral(token.lex.parse().unwrap()),
        TokenType::Identifier => Node::Identifier(token.lex.clone()),
        TokenType::BinaryOperator => Node::BinaryOperator(token.lex.clone()),
        _ => Node::Panic,
    };

    if expr == Node::Panic {
        return Err(format!(
            "Invalid token while parsing a primary: {:?}",
            token
        ));
    }

    Ok(ParseMessage(1, expr))
}

fn parse_additive(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    let mut left = parse_primary(tokens.first().unwrap()).unwrap().1;
    let mut consumed = 1;

    while tokens
        .get(consumed)
        .is_some_and(|tk| tk.kind == TokenType::BinaryOperator)
    {
        let operator = parse_primary(&tokens[consumed]).unwrap().1;
        let dealing_with_highcalc = tokens
            .get(consumed + 2)
            .is_some_and(|tk| tk.lex == "*" || tk.lex == "/");
        let right = if !dealing_with_highcalc {
            parse_primary(&tokens[consumed + 1]).unwrap()
        } else {
            parse_highcalc(&tokens[consumed + 1..].to_vec()).unwrap()
        };

        left = parse_binary_expr(&left, &operator, &right.1).unwrap().1;

        consumed += 1 + right.0;
        println!("{consumed}");

        println!("{:?}", left);
    }

    Ok(ParseMessage(consumed, left))
}

fn parse_highcalc(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    let mut left = parse_primary(&tokens[0]).unwrap().1;
    let mut consumed = 1;

    while tokens
        .get(consumed)
        .is_some_and(|tk| tk.lex == "*" || tk.lex == "/")
    {
        let operator = parse_primary(&tokens[consumed]).unwrap().1;
        let right = parse_primary(&tokens[consumed + 1]).unwrap().1;
        left = parse_binary_expr(&left, &operator, &right).unwrap().1;

        consumed += 2;
    }

    Ok(ParseMessage(consumed, left))
}

fn parse_variable_declaration(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    let declaration_type = Node::DeclarationType(tokens[0].clone().kind);
    let var_name = tokens[1].clone().lex;

    if tokens[2].kind != TokenType::Equals {
        return Err(format!(
            "Unexpected token while parsing variable declaration: ({:?})",
            tokens[1]
        ));
    }

    let semilicon_index = tokens[0..tokens.len()]
        .iter()
        .position(|tk| tk.kind == TokenType::Semilicon)
        .unwrap();

    let expr_msg = parse_expression(&tokens[3..semilicon_index].to_vec()).unwrap();

    let delete_amount = expr_msg.0;
    let expr_node = expr_msg.1;

    let declaration =
        Node::VariableDeclaration(declaration_type.to_box(), var_name, expr_node.to_box());

    Ok(ParseMessage(delete_amount + 4, declaration))
}

fn parse_function_declaration(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    let function_name = tokens.get(1).unwrap().lex.to_owned();

    let arguments_range = 3..tokens
        .iter()
        .position(|tk| tk.kind == TokenType::CloseParenthesis)
        .unwrap();

    let params = parse_function_parameters(&tokens[arguments_range].to_vec()).unwrap();
    let (consumed_params_tokens, params) = params;

    let code_range = (consumed_params_tokens + 5)
        ..tokens
            .iter()
            .position(|tk| tk.kind == TokenType::CloseCurly)
            .unwrap();

    let mut function_tokens = tokens[code_range].to_vec();
    function_tokens.push(Token::new(TokenType::EOF, "EOF".to_string()));

    let code_token_count = function_tokens.len();
    let function_code = parse(function_tokens).to_box();

    let declaration_node = Node::FunctionDeclaration(
        function_name,
        params.iter().map(|node| node.to_box()).collect(),
        function_code,
    );

    Ok(ParseMessage(
        consumed_params_tokens + code_token_count + 6 - 1,
        declaration_node,
    ))
}

fn parse_function_parameters(tokens: &Vec<Token>) -> Result<(usize, Vec<Node>), String> {
    if tokens.len() == 0 {
        return Ok((0, Vec::<Node>::new()));
    }

    let mut consumed: usize = 0;
    let mut params: Vec<Node> = Vec::new();

    loop {
        let name = tokens[consumed].lex.to_owned();

        params.push(Node::FunctionParameter(name));

        if consumed >= tokens.len() - 1 {
            consumed += 1;
            break;
        }
        consumed += 2;
    }

    Ok((consumed, params))
}

fn parse_function_call(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    if tokens.len() < 3 {
        return Err(format!(
            "Function call should be at least 3 tokens long, given one is: {} tokens long",
            tokens.len()
        ));
    }

    let function_name = tokens.first().unwrap().lex.to_owned();
    let arguments_range = 2..(tokens.len() - 1);

    let arguments_tokens = tokens[arguments_range].to_vec();
    let arguments = parse_arguments(&arguments_tokens).unwrap().to_box();

    let identifier_node = Node::Identifier(function_name).to_box();

    Ok(ParseMessage(
        tokens.len(),
        Node::FunctionCall(identifier_node, arguments),
    ))
}

fn parse_arguments(tokens: &Vec<Token>) -> Result<Node, String> {
    if tokens.len() == 0 {
        return Ok(Node::Arguments(Vec::<Box<Node>>::new()));
    }
    println!("hi bro, {:?}", tokens);
    let mut parenthesis_state = 0;
    let mut arguments = Vec::<Box<Node>>::new();
    let mut index = 0;

    for (i, tk) in tokens.iter().enumerate() {
        parenthesis_state += match tk.kind {
            TokenType::OpenCurly | TokenType::OpenParenthesis => 1,
            TokenType::CloseCurly | TokenType::CloseParenthesis => -1,
            _ => 0,
        };

        if parenthesis_state == 0 && (tk.kind == TokenType::Comma || i == tokens.len() - 1) {
            println!("{} {}", index, i);
            let expr_end = i + if i == tokens.len() - 1 { 1 } else { 0 };

            let expr_msg = parse_expression(&tokens[index..expr_end].to_vec()).unwrap();
            let expr = expr_msg.1;

            index = i + 1;

            arguments.push(expr.to_box());
        }
    }

    Ok(Node::Arguments(arguments))
}

fn parse_return(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    let expr_msg = parse_expression(&tokens[1..].to_vec()).unwrap();
    let consumed = expr_msg.0;
    let expr = expr_msg.1;

    Ok(ParseMessage(
        consumed + 1,
        Node::ReturnStatement(expr.to_box()),
    ))
}
