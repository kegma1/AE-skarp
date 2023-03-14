use anyhow::Result;
use std::{io::Lines, iter::Peekable, str::Chars};
use std::io::{BufReader, Read};

#[derive(Debug)]
pub enum Op {
    SumInt,
}

#[derive(Debug)]
pub enum Node {
    PushInt(i64),
    Operator(Op),
    Identifier(String),
}

pub struct Parser<R: Read> {
    lines: Lines<BufReader<R>>,
    ast: Vec<Node>,
}

impl<R: Read> Parser<R> {
    pub fn parse(lines: Lines<BufReader<R>>) -> Result<Vec<Node>> {
        let mut parser = Parser { lines, ast: vec![] };

        while let Some(Ok(line)) = parser.lines.next() {
            parser.parse_line(&mut line.chars().peekable())?;
        }

        Ok(parser.ast)
    }

    fn parse_line(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
        loop {
            match line.peek() {
                Some('0'..='9') => self.parse_number(line)?,
                Some(_) => {
                    let _ = line.next().unwrap();
                }
                None => break,
            }
        }
        Ok(())
    }

    fn parse_number(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
        let mut num_string = String::from("");
        loop {
            match line.peek() {
                Some('0'..='9') => num_string.push(line.next().unwrap()),
                _ => break,
            }
        }

        self.ast.push(Node::PushInt(num_string.parse()?));
        Ok(())
    }

    fn parse_operator(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
        Ok(())
    }

    fn parse_identifier(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
        Ok(())
    }
}
