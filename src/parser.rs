use anyhow::{Result, anyhow};
use std::{io::Lines, iter::Peekable, str::Chars};
use std::io::{BufReader, Read};

#[derive(Debug)]
pub enum Op {
    SumInt,
    SubInt,
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
        let mut word = String::from("");
        loop {
            match line.peek() {
                Some('0'..='9') => self.parse_number(line)?,
                Some(c) if c.is_whitespace() => {
                    self.parse_word(&word)?;
                    word = String::from("");
                    line.next();
                },
                Some(_) => word.push(line.next().unwrap()),
                None => {self.parse_word(&word)?; break;},
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

    fn parse_word(&mut self, word: &String) -> Result<()> {
        if word.is_empty() {
            return Ok(());
        }

        if let Ok(_) = self.parse_operator(word) {
            Ok(())
        } else {
            self.parse_identifier(word)?;
            Ok(())
        }

    }

    fn parse_operator(&mut self, word: &String) -> Result<()> {
        match word.as_str() {
            "+" => {
                self.ast.push(Node::Operator(Op::SumInt));
                Ok(())
            },
            "-" => {
                self.ast.push(Node::Operator(Op::SubInt));
                Ok(())
            },
            _ => Err(anyhow!("Unknown operator: {}", word)),
        }
    }

    fn parse_identifier(&mut self, word: &String) -> Result<()> {
        self.ast.push(Node::Identifier(word.clone()));
        Ok(())
    }
}
