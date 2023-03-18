use anyhow::{anyhow, Result};
use std::{iter::Peekable, str::Chars};

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
    If {
        condition: Vec<Node>,
        block: Vec<Node>,
    },
}

pub struct Parser<'a> {
    code: Peekable<Chars<'a>>,
    ast: Vec<Node>,
}

impl Parser<'_> {
    pub fn parse(code: Peekable<Chars>) -> Result<Vec<Node>> {
        let mut parser = Parser { code, ast: vec![] };
        loop {
            match parser.code.peek() {
                Some('0'..='9') => parser.parse_number()?,
                Some(c) if c.is_whitespace() => {
                    parser.code.next();
                }
                Some(_) => parser.parse_word()?,
                None => break,
            }
        }
        Ok(parser.ast)
    }

    fn parse_number(&mut self) -> Result<()> {
        let mut num_string = String::from("");
        loop {
            match self.code.peek() {
                Some('0'..='9') => num_string.push(self.code.next().unwrap()),
                _ => break,
            }
        }

        self.ast.push(Node::PushInt(num_string.parse()?));
        Ok(())
    }

    fn parse_word(&mut self) -> Result<()> {
        let mut word = String::from("");

        loop {
            match self.code.peek() {
                Some(c) if c.is_whitespace() => break,
                Some(_) => word.push(self.code.next().unwrap()),
                None => break,
            }
        }

        if word.is_empty() {
            return Ok(());
        }

        if let Ok(_) = self.parse_operator(&word) {
            Ok(())
        } else if let Ok(_) = self.parse_keyword(&word) {
            Ok(())
        } else {
            self.parse_identifier(&word)?;
            Ok(())
        }
    }

    fn parse_keyword(&mut self, word: &String) -> Result<()> {
        match word.as_str() {
            "hvis" => {
                let condition = self.parse_condition()?;
                let block = self.parse_block()?;

                self.ast.push(Node::If {
                    condition,
                    block: block,
                });

                Ok(())
            }
            _ => Err(anyhow!("Unknown keyword: {}", word)),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Node>> {
        let mut block = String::from("");
        if let Some('{') = self.code.peek() {
            self.code.next();
            let mut bracket_count = 1;
            while bracket_count > 0 {
                match self.code.next() {
                    Some('{') => {
                        bracket_count += 1;
                        block.push('{')
                    }
                    Some('}') => {
                        bracket_count -= 1;
                        if bracket_count != 0 {
                            block.push('}')
                        }
                    }
                    Some(x) => {
                        block.push(x);
                    }
                    None => return Err(anyhow!("no ending bracket found")),
                };
            }
        }
        Parser::parse(block.chars().peekable())
    }

    fn parse_condition(&mut self) -> Result<Vec<Node>> {
        let mut condition = String::from("");
        loop {
            match self.code.peek() {
                Some('{') => break,
                Some(_) => condition.push(self.code.next().unwrap()),
                None => return Err(anyhow!("Condition must be on one line")),
            }
        }
        Parser::parse(condition.chars().peekable())
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
