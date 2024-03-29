mod eval;
mod parser;
mod utils;

use anyhow::Result;
use parser::Parser;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};


fn main() -> Result<()> {
    let debug_flag: String = String::from("-d");
    let debug_mode:bool;
    let args = env::args().collect::<Vec<String>>();
    let path: Option<&String>;

    if Some(&debug_flag) == args.get(1){
        path = args.get(2);
        debug_mode = true
    } else {
        path = args.get(1);
        debug_mode = false
    }

    if let Some(p) = path {
        let mut source_code: String = String::from("");
        BufReader::new(File::open(p)?).read_to_string(&mut source_code)?;
        let ast = Parser::parse(source_code.chars().peekable(), None)?;
        if debug_mode{
            for (i, node) in ast.ast.iter().enumerate() {
                println!("{}: {:?}", i, node)
            }
            println!("\n");
        }
    
        eval::eval(ast.ast)?;
    } else {
        println!("no argument given")
    }
    Ok(())
}
