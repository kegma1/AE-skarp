mod lexer;

use lexer::lex;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let path: Option<&String> = args.get(1);

    if let Some(p) = path {
        let source_code = fs::read_to_string(p).expect(&format!("Kunne ikke lese fil '{}'", p));
        println!("{:?}", lex(&String::from(source_code + " ")).unwrap())
    } else {
        println!("no argument given")
    }
}
