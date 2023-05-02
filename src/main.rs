mod parser;
mod utils;

use anyhow::Result;
use parser::Parser;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let path: Option<&String> = args.get(1);
    if let Some(p) = path {
        let mut source_code: String = String::from("");
        BufReader::new(File::open(p)?).read_to_string(&mut source_code)?;
        let ast = Parser::parse(source_code.chars().peekable(), None)?;
        for node in ast.ast {
            println!("{:?}", node)
        }
    } else {
        println!("no argument given")
    }
    Ok(())
}
