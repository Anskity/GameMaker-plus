use crate::string_utils::{char_at, is_alphabetic, is_numeric};
use std::collections::HashMap;

pub struct Keywords {}
impl Keywords {
    pub fn get<'a>() -> HashMap<&'a str, TokenType> {
        let mut map = HashMap::new();
        map.insert("let", TokenType::Let);
        map.insert("const", TokenType::Const);
        map.insert("function", TokenType::Function);
        map.insert("return", TokenType::Return);

        map
    }
}

pub struct SinglecharTokens {}
impl SinglecharTokens {
    pub fn get() -> HashMap<char, TokenType> {
        let mut map = HashMap::new();

        map.insert(';', TokenType::Semilicon);
        map.insert('=', TokenType::Equals);
        map.insert('(', TokenType::OpenParenthesis);
        map.insert(')', TokenType::CloseParenthesis);
        map.insert('+', TokenType::BinaryOperator);
        map.insert('-', TokenType::BinaryOperator);
        map.insert('*', TokenType::BinaryOperator);
        map.insert('/', TokenType::BinaryOperator);
        map.insert('{', TokenType::OpenCurly);
        map.insert('}', TokenType::CloseCurly);
        map.insert(',', TokenType::Comma);

        map
    }
}

pub struct SkippableCharacters {}
impl SkippableCharacters {
    pub fn get() -> Vec<char> {
        vec![' ', '\t', '\r', '\n']
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Equals,
    BinaryOperator,
    Semilicon,
    OpenParenthesis,
    CloseParenthesis,

    Identifier,
    NumericListeral,
    Let,
    Const,
    Function,
    Return,

    Comma,

    OpenCurly,
    CloseCurly,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lex: String,
}

impl Token {
    pub fn new(kind: TokenType, lex: String) -> Token {
        Token { kind, lex }
    }

    pub fn to_box(&self) -> Box<Self> {
        Box::new(self.to_owned())
    }
}

use crate::box_utils::Boxable;

impl Boxable for Token {}
impl Boxable for TokenType {}

pub fn tokenize(mut src: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    println!("{}", src);

    while src.chars().count() != 0 {
        let char = char_at(&src, 0);
        let char_as_string = String::from(char);

        if is_alphabetic(&char_as_string) || char == '_' {
            let mut identifier = String::new();

            loop {
                identifier.push(src.remove(0));

                if src.len() == 0 {
                    break;
                }

                let next_char = char_at(&src, 0);

                if !is_alphabetic(&next_char.to_string())
                    && next_char != '_'
                    && !is_numeric(&next_char.to_string())
                {
                    break;
                }
            }

            let keywords = Keywords::get();
            let is_keyword = keywords.contains_key(identifier.as_str());
            let token_type: TokenType = if is_keyword {
                keywords.get(identifier.as_str()).cloned().unwrap()
            } else {
                TokenType::Identifier
            };

            tokens.push(Token::new(token_type, identifier));
        } else if is_numeric(&char_as_string) {
            let mut literal = String::new();

            loop {
                literal.push(src.remove(0));

                if src.len() == 0 {
                    break;
                }

                let next_char = String::from(char_at(&src, 0));

                if !is_numeric(&next_char) {
                    break;
                }
            }

            tokens.push(Token::new(TokenType::NumericListeral, literal));
        } else if SinglecharTokens::get().contains_key(&char) {
            let kind = SinglecharTokens::get().get(&char).cloned().unwrap();

            tokens.push(Token::new(kind, char_as_string));

            src.remove(0);
        } else if SkippableCharacters::get().contains(&char) {
            src.remove(0);
        } else {
            panic!("Strange character: {:?}", char);
        }
    }

    tokens.push(Token::new(TokenType::EOF, "EOF".to_string()));

    tokens
}
