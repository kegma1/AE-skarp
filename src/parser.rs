use anyhow::{anyhow, Result};
use std::io::{BufReader, Read, BufRead};
use std::{io::Lines, iter::Peekable, str::Chars};

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
    If{condition: Vec<Node>, block: Vec<Node>}
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
                Some(c) if c.is_whitespace() => {
                    line.next();
                }
                Some(_) => self.parse_word(line)?,
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

    fn parse_word(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
        let mut word = String::from("");

        loop {
            match line.peek() {
                Some(c) if c.is_whitespace() => break,
                Some(_) => word.push(line.next().unwrap()),
                None => break,
            }
        }

        if word.is_empty() {
            return Ok(());
        }

        if let Ok(_) = self.parse_operator(&word) {
            Ok(())
        } else if let Ok(_) = self.parse_keyword(&word, line) {
            Ok(())
        } else {
            self.parse_identifier(&word)?;
            Ok(())
        }
    }

    fn parse_keyword(&mut self, word: &String, line: &mut Peekable<Chars>) -> Result<()> {
        match word.as_str() {
            "hvis" => {
                let condition = self.parse_condition(line)?;
                let block = self.parse_block(line)?;
    
                self.ast.push(Node::If { condition, block: block });
    
                Ok(())
            }
            _ => Err(anyhow!("Unknown keyword: {}", word)),
        }
    }  

    fn parse_block(&mut self, line: &mut Peekable<Chars>) -> Result<Vec<Node>> {
        let mut block = String::new();

        Parser::parse(BufReader::new(block.as_bytes()).lines())
    }

    fn parse_condition(&mut self, line: &mut Peekable<Chars>)  -> Result<Vec<Node>> {
        let mut condition = String::from("");
        loop {
            match line.peek() {
                Some('{') => break,
                Some(_) => condition.push(line.next().unwrap()),
                None => return Err(anyhow!("Condition must be on one line"))
            }
        }
        Parser::parse(BufReader::new(condition.as_bytes()).lines())
    }
   

    fn parse_operator(&mut self, word: &String) -> Result<()> {
        match word.as_str() {
            "+" => {
                self.ast.push(Node::Operator(Op::SumInt));
                Ok(())
            }
            "-" => {
                self.ast.push(Node::Operator(Op::SubInt));
                Ok(())
            }
            _ => Err(anyhow!("Unknown operator: {}", word)),
        }
    }

    fn parse_identifier(&mut self, word: &String) -> Result<()> {
        self.ast.push(Node::Identifier(word.clone()));
        Ok(())
    }
}
