mod parser;

use anyhow::Result;
use parser::Parser;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let path: Option<&String> = args.get(1);

    if let Some(p) = path {
        let source_code = BufReader::new(File::open(p)?).lines();
        let ast = Parser::parse(source_code)?;
        for node in ast {
            println!("{:?}", node)
        }
    } else {
        println!("no argument given")
    }
    Ok(())
}
