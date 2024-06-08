use crate::compiler::tokenizer::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    BinaryExpression(Box<Node>, Box<Node>, Box<Node>),
    NumericLiteral(i32),
    Identifier(String),
    BinaryOperator(String),
    Program(Vec<Box<Node>>),
    Panic,
    VariableDeclaration(Box<Node>, String, Box<Node>),
    DeclarationType(TokenType),
}

impl Node {
    pub fn display_program(self: &Node, indent: usize) {
        const INDENT_SIZE: usize = 2;
        let indent_space = " ".repeat(indent * INDENT_SIZE);
        match self {
            Node::Program(body) => {
                println!("{}Program {{{indent}", indent_space);
                for node in body {
                    node.display_program(indent + 1);
                }
                println!("{}}}{indent}", indent_space);
            }
            Node::BinaryExpression(left, operator, right) => {
                println!("{}Binary Expression {{{indent}", indent_space);
                left.display_program(indent + 1);
                operator.display_program(indent + 1);
                right.display_program(indent + 1);
                println!("{}}}{indent}", indent_space)
            }
            Node::Panic => panic!("PANIC"),
            Node::NumericLiteral(numb) => {
                println!("{}NumericLiteral: {}", indent_space, numb)
            }
            Node::Identifier(id) => {
                println!("{}Identifier: {}", indent_space, id)
            }
            Node::BinaryOperator(operator) => {
                println!("{}Binary Operator: {}", indent_space, operator)
            }
            Node::VariableDeclaration(declaration_type, name, expression) => {
                println!("{indent_space}Variable Declaration {{{indent}");
                declaration_type.display_program(indent + 1);
                println!(
                    "{}Name: {}",
                    indent_space.clone() + " ".repeat(INDENT_SIZE.clone()).as_str(),
                    name
                );
                println!(
                    "{}Expression: {{{}",
                    indent_space.to_string() + " ".repeat(INDENT_SIZE).as_str(),
                    indent + 1
                );
                expression.display_program(indent + 2);
                println!(
                    "{}}}{}",
                    indent_space.to_string() + " ".repeat(INDENT_SIZE).as_str(),
                    indent + 1
                );
                println!("{indent_space}}}{indent}");
            }
            Node::DeclarationType(token) => println!("{indent_space}DeclarationType: {:?}", token),
        }
    }

    pub fn to_box(&self) -> Box<Node> {
        Box::new(self.to_owned())
    }
}
