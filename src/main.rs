mod lexer;

use std::env;
use std::fs;
use lexer::lex;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let path: Option<&String> = args.get(1);

    if let Some(p) = path {
        let source_code = fs::read_to_string(p)
            .expect(&format!("Kunne ikke lese fil '{}'", p));
        println!("{:?}", lex(&source_code))
    } else {
        println!("no argument given")
    }
}

