use anyhow::{anyhow, Result};
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Op {
    SumInt,
    SubInt,
    MultInt,
    DivInt,
    ModInt,

    EqInt,
    LtInt,
    GtInt,
    LqInt,
    GqInt,

    Dup,

    Println,
}

#[derive(Debug)]
pub enum Node {
    PushInt(i64),
    PushBool(bool),
    Operator(Op),
    Identifier(String),
    If {
        condition: Vec<Node>,
        block: Vec<Node>,
    },
    Else {
        block: Vec<Node>,
    },
    While {
        condition: Vec<Node>,
        block: Vec<Node>,
    }
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
            return Ok(())
        } 

        let keyword_res = self.parse_keyword(&word)?;
        if keyword_res == true {
            return Ok(())
        }
        self.parse_identifier(&word)?;
            Ok(())
    }

    fn parse_keyword(&mut self, word: &String) -> Result<bool> {
        match word.as_str() {
            "usann" => {
                self.ast.push(Node::PushBool(false));
                Ok(true)
            }
            "sann" => {
                self.ast.push(Node::PushBool(true));
                Ok(true)
            }
            "når" => {
                let condition = self.parse_condition()?;
                let block = self.parse_block()?;

                self.ast.push(Node::While {
                    condition,
                    block,
                });

                Ok(true)
            }
            "hvis" => {
                let condition = self.parse_condition()?;
                let block = self.parse_block()?;

                self.ast.push(Node::If {
                    condition,
                    block,
                });

                Ok(true)
            }
            "ellers" => {
                while self.code.peek().unwrap().is_whitespace() {
                    self.code.next();
                }
                if let Some(Node::If { condition: _, block: _ }) = self.ast.last(){
                    let block = self.parse_block()?;
                    self.ast.push(Node::Else {
                        block,
                    });
    
                    Ok(true)
                } else {
                    Err(anyhow!("Else block can only end if block"))
                }
                
            }
            _ => Ok(false),
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
                    None => return Err(anyhow!("No ending bracket found")),
                };
            }
        } else {
            return Err(anyhow!("No block found"))
        }
        Parser::parse(block.chars().peekable())
    }

    fn parse_condition(&mut self) -> Result<Vec<Node>> {
        let mut condition = String::from("");
        loop {
            match self.code.peek() {
                Some('{') => break,
                Some(_) => condition.push(self.code.next().unwrap()),
                None => return Err(anyhow!("No block found")),
            }
        }
        let condition_ast = Parser::parse(condition.chars().peekable())?;
        if !condition_ast.is_empty() {
            Ok(condition_ast)
        } else {
            Err(anyhow!("no condition found"))
        }
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
            "*" => {
                self.ast.push(Node::Operator(Op::MultInt));
                Ok(())
            }
            "/" => {
                self.ast.push(Node::Operator(Op::DivInt));
                Ok(())
            }
            "%" => {
                self.ast.push(Node::Operator(Op::ModInt));
                Ok(())
            }
            "==" => {
                self.ast.push(Node::Operator(Op::EqInt));
                Ok(())
            }
            "<" => {
                self.ast.push(Node::Operator(Op::LtInt));
                Ok(())
            }
            ">" => {
                self.ast.push(Node::Operator(Op::GtInt));
                Ok(())
            }
            "<=" => {
                self.ast.push(Node::Operator(Op::LqInt));
                Ok(())
            }
            ">=" => {
                self.ast.push(Node::Operator(Op::GqInt));
                Ok(())
            }
            "dup" => {
                self.ast.push(Node::Operator(Op::Dup));
                Ok(())
            }
            "skrivnl" => {
                self.ast.push(Node::Operator(Op::Println));
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