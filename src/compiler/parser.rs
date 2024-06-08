use crate::compiler::ast::Node;
use crate::compiler::tokenizer::{Token, TokenType};

struct ParseMessage(usize, Node);

pub fn parse(mut tokens: Vec<Token>) -> Node {
    let mut statements: Vec<Node> = Vec::new();

    while tokens.first().unwrap().kind != TokenType::EOF {
        let parse_message = parse_statement(&tokens);
        let ParseMessage(delete_amount, result) = parse_message.unwrap();

        tokens.drain(0..delete_amount);
        statements.push(result);
    }

    let boxxer = |stmt: &Node| Box::new(stmt.clone());

    let boxed_statements: Vec<Box<Node>> = statements.iter().map(boxxer).collect();

    Node::Program(boxed_statements)
}

fn parse_statement(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
    let first_token = tokens.first().unwrap();
    match first_token.kind {
        TokenType::Let | TokenType::Const => parse_variable_declaration(tokens),
        _ => Err(format!(
            "Unexpected token while parsing statement: \"{}\" | {}",
            first_token.lex, first_token.lex
        )),
    }
}

fn parse_expression(tokens: &Vec<Token>) -> Result<ParseMessage, String> {
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

    while tokens[consumed].kind == TokenType::BinaryOperator {
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

    let expr_msg = parse_expression(&tokens[3..tokens.len()].to_vec()).unwrap();

    let delete_amount = expr_msg.0;
    let expr_node = expr_msg.1;

    let declaration =
        Node::VariableDeclaration(declaration_type.to_box(), var_name, expr_node.to_box());

    Ok(ParseMessage(delete_amount + 3, declaration))
}
