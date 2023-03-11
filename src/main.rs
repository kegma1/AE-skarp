mod common;
// mod interpret;
mod lexer;
mod parser;

// use interpret::interpret;
use lexer::lex;
use parser::parse;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let path: Option<&String> = args.get(1);

    if let Some(p) = path {
        let source_code = fs::read_to_string(p).expect(&format!("Kunne ikke lese fil '{}'", p));
        let lexed = lex(&String::from(source_code + " ")).unwrap();
        let (parsed, stack) = parse(lexed, None).unwrap();
        println!("{:?}", stack);
        print_ast(&parsed);
        // let _ = interpret(parsed);
    } else {
        println!("no argument given")
    }
}

fn print_ast(ast: &Vec<parser::Node>) {
    for (i, op) in ast.iter().enumerate() {
        println!("{}: {:?}", i, op)
    }
}
