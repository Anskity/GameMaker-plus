use compiler::tokenizer::{tokenize, Token};
use gamemaker_plus::compiler;

use front_end::{get_source_code, parse_arguments};
use gamemaker_plus::front_end;

use compiler::parser::parse;

fn main() {
    let config = parse_arguments().unwrap();

    println!("Path: {}\nStrict: {}\n\n", config.path, config.strict);

    let source_code = get_source_code(&config).expect("Couldn't read the source code");

    let tokens: Vec<Token> = tokenize(source_code);
    println!("Tokens: {:?}", tokens);
    let ast = parse(tokens);

    ast.display_program(0);
}
