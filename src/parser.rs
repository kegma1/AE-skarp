use anyhow::{anyhow, Result};
use std::{iter::Peekable, str::Chars};
use crate::utils::*;

pub struct Parser<'a> {
    code: Peekable<Chars<'a>>,
    pub ast: Vec<Node>,
    pub type_stack: Vec<Type>,
}

impl Parser<'_> {
    pub fn parse(code: Peekable<Chars>, context: Option<Vec<Type>>) -> Result<Parser> {
        let mut parser = Parser {
            code,
            ast: vec![],
            type_stack: if let Some(ctx) = context {
                ctx
            } else {
                vec![]
            },
        };
        loop {
            match parser.code.peek() {
                Some('0'..='9') => parser.parse_number()?,
                Some('-') => parser.parse_number()?,
                Some(c) if c.is_whitespace() => {
                    parser.code.next();
                }
                Some(_) => parser.parse_word(None)?,
                None => break,
            }
        }
        Ok(parser)
    }

    fn parse_number(&mut self) -> Result<()> {
        let mut num_string = String::from("");
        loop {
            match self.code.peek() {
                Some('-') => {
                    num_string.push(self.code.next().unwrap());
                    if let Some('0'..='9') = self.code.peek() {
                    } else {
                        return self.parse_word(Some(num_string));
                    }
                }
                Some('0'..='9') => num_string.push(self.code.next().unwrap()),
                _ => break,
            }
        }
        self.ast.push(Node::PushInt(num_string.parse()?));
        self.type_stack.push(Type::Int);
        Ok(())
    }

    fn parse_word(&mut self, optional_start: Option<String>) -> Result<()> {
        let mut word = if let Some(start) = optional_start {
            start
        } else {
            String::from("")
        };

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

        let operator_res = self.parse_operator(&word)?;
        if operator_res == true {
            return Ok(());
        }

        let keyword_res = self.parse_keyword(&word)?;
        if keyword_res == true {
            return Ok(());
        }
        self.parse_identifier(&word)?;
        Ok(())
    }

    fn parse_keyword(&mut self, word: &String) -> Result<bool> {
        match word.as_str() {
            "usann" => {
                self.ast.push(Node::PushBool(false));
                self.type_stack.push(Type::Bool);
                Ok(true)
            }
            "sann" => {
                self.ast.push(Node::PushBool(true));
                self.type_stack.push(Type::Bool);
                Ok(true)
            }
            "når" => {
                let condition = self.parse_condition()?;
                let block = self.parse_block()?;

                self.ast.push(Node::While { condition, block });

                Ok(true)
            }
            "hvis" => {
                let condition = self.parse_condition()?;
                let block = self.parse_block()?;

                self.ast.push(Node::If { condition, block });

                Ok(true)
            }
            "ellers" => {
                while self.code.peek().unwrap().is_whitespace() {
                    self.code.next();
                }
                if let Some(Node::If {
                    condition: _,
                    block: _,
                }) = self.ast.last()
                {
                    let block = self.parse_block()?;
                    self.ast.push(Node::Else { block });

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
            return Err(anyhow!("No block found"));
        }
        Ok(Parser::parse(block.chars().peekable(), Some(self.type_stack.clone()))?.ast)
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
        let condition_ast = Parser::parse(condition.chars().peekable(), Some(self.type_stack.clone()))?.ast;
        if !condition_ast.is_empty() {
            Ok(condition_ast)
        } else {
            Err(anyhow!("no condition found"))
        }
    }

    fn parse_operator(&mut self, word: &String) -> Result<bool> {
        match word.as_str() {
            "+" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{ op: Op::SumInt, arity: 2, func: |args| match (args.get(0), args.get(1)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Int(a + b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Int)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "-" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{ op: Op::SubInt, arity: 2, func: |args| match (args.get(0), args.get(1)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Int(a - b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Int)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "*" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::MultInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Int(a * b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Int)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "/" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::DivInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Int(a / b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Int)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "%" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::ModInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Int(a % b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Int)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "==" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::EqInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Bool(a == b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Bool)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "<" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::LtInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Bool(a < b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Bool)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            ">" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::GtInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Bool(a > b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Bool)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "<=" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::LqInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Bool(a <= b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Bool)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            ">=" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator{op:Op::GqInt, arity:2, func:|args| match (args.get(0), args.get(0)) {
                            (Some(Value::Int(a)), Some(Value::Int(b))) => Some(vec![Value::Bool(a >= b)]),
                            _ => None,
                        }});
                        self.type_stack.push(Type::Bool)
                    }
                    (_,_) => return Err(anyhow!("{} operator does not support {:?} and {:?}", word, a, b))
                }
                Ok(true)
            }
            "dup" => {
                let Some(b) = self.type_stack.pop() else {
                    return Err(anyhow!("{} needs ateast 1 argument", word));
                };
                self.ast.push(Node::Operator { op: Op::Dup, arity: 1, func: |args| Some(vec![args[0].clone(), args[0].clone()]) });
                self.type_stack.push(b);
                self.type_stack.push(b);
                Ok(true)
            }
            "skrivnl" => {
                let _ = self.type_stack.pop().unwrap();
                self.ast.push(Node::Operator { op: Op::Println, arity: 1, func: |args| {
                    println!("{}", args[0]);
                    None
                } });
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn parse_identifier(&mut self, word: &String) -> Result<()> {
        self.ast.push(Node::Identifier(word.clone()));
        Ok(())
    }
}
