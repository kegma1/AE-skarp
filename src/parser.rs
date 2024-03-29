use crate::utils::*;
use anyhow::{anyhow, Result, Ok};
use std::{iter::Peekable, str::Chars};

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
            type_stack: if let Some(ctx) = context { ctx } else { vec![] },
        };
        loop {
            match parser.code.peek() {
                Some('0'..='9') => parser.parse_number()?,
                Some('-') => parser.parse_number()?,
                Some(c) if c.is_whitespace() => {
                    parser.code.next();
                }
                Some('"') => parser.parse_string()?,
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

    fn parse_string(&mut self) -> Result<()> {
        self.code.next();
        let mut string = "".to_string();
        loop {
            match self.code.peek() {
                Some('"') => break,
                Some(_) => string.push(self.code.next().unwrap()),
                None => return Err(anyhow!("String not closed"))
            }
        }
        self.code.next();
        self.ast.push(Node::PushStr(string));
        self.type_stack.push(Type::Str);
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
                let mut condition = self.parse_condition()?;
                let mut block = self.parse_block()?;
                let condition_start = -(block.len() as isize + condition.len() as isize + 2);

                self.ast.append(&mut condition);
                self.ast
                    .push(Node::JumpIfFalse(JumpPointer::new(block.len() as isize + 1)));

                self.ast.append(&mut block);
                self.ast.push(Node::Jump(JumpPointer::new(condition_start)));

                Ok(true)
            }
            "hvis" => {
                let mut condition = self.parse_condition()?;
                let mut block = self.parse_block()?;

                self.ast.append(&mut condition);
                self.ast
                    .push(Node::JumpIfFalse(JumpPointer::new(block.len() as isize + 1)));

                self.ast.append(&mut block);
                self.ast.push(Node::EndOfIf);

                Ok(true)
            }
            "ellvis" => {
                if let Some(Node::EndOfIf) = self.ast.last() {
                    self.ast.pop();
                    let mut condition = self.parse_condition()?;
                    let mut block = self.parse_block()?;
                    self.ast.push(Node::Jump(JumpPointer::new((condition.len() + block.len()) as isize + 1)));
    
                    self.ast.append(&mut condition);
                    self.ast
                        .push(Node::JumpIfFalse(JumpPointer::new(block.len() as isize + 1)));
    
                    self.ast.append(&mut block);
                    self.ast.push(Node::EndOfIf);
    
                    Ok(true)
                } else {
                    Err(anyhow!("Elif block can only end if block"))
                }
            }
            "ellers" => {
                self.remove_whitespace();
                if let Some(Node::EndOfIf) = self.ast.last() {
                    let mut block = self.parse_block()?;
                    self.ast.pop();
                    self.ast
                        .push(Node::Jump(JumpPointer::new(block.len() as isize)));
                    self.ast.append(&mut block);

                    Ok(true)
                } else {
                    Err(anyhow!("Else block can only end if block"))
                }
            }
            "var" => {
                let name = if let Node::Identifier(name) = self.get_name()? {
                    Some(name)
                } else {
                    None
                };

                self.remove_whitespace();
                
                if self.code.peek().unwrap() != &'=' {
                    return Err(anyhow!("Expected '=' after constant name, but found '{}'", self.code.peek().unwrap()))
                } else {
                    self.code.next();
                }

                self.remove_whitespace();

                let mut value = if self.code.peek() == Some(&'{') {
                    self.parse_block()?
                } else {
                    let mut exper = "".to_string();

                    while self.code.peek() != Some(&'\n'){
                        exper.push(self.code.next().unwrap())
                    }
                    let exper_parser = Parser::parse(exper.chars().peekable(), Some(self.type_stack.clone()))?;
                    exper_parser.ast
                };

                self.ast.push(Node::DefineConst(name.clone().expect("Identifier not found")));
                self.ast.append(&mut value);
                self.ast.push(Node::Return(name.clone().expect("Identifier not found")));
                Ok(true)

            }
            _ => Ok(false),
        }
    }

    fn remove_whitespace(&mut self) {
        while self.code.peek().unwrap().is_whitespace() {
            self.code.next();
        }
    }

    fn get_name(&mut self) -> Result<Node> {
        self.remove_whitespace();
        let mut name = "".to_string();
        while !self.code.peek().unwrap().is_whitespace() {
            name.push(self.code.next().unwrap())
        }

        let _ = self.parse_word(Some(name));

        if let Some(Node::Identifier(_))= self.ast.last() {
            Ok(self.ast.pop().unwrap())
        } else {
            Err(anyhow!("Expected identifier but found '{:?}'", self.ast.pop().unwrap()))
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
        let mut condition_parser =
            Parser::parse(condition.chars().peekable(), Some(self.type_stack.clone()))?;
        let condition_ast = condition_parser.ast;

        if let Some(Type::Bool) = condition_parser.type_stack.pop() {
            if condition_parser.type_stack != self.type_stack {
                return Err(anyhow!(
                    "condition must only leave a bool at the top of the stack"
                ));
            }

            if !condition_ast.is_empty() {
                Ok(condition_ast)
            } else {
                Err(anyhow!("no condition found"))
            }
        } else {
            Err(anyhow!(
                "condition must leave a bool on the top of the stack"
            ))
        }
    }

    fn parse_operator(&mut self, word: &String) -> Result<bool> {
        match word.as_str() {
            "+" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::SumInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Int(a + b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Int)
                    }
                    (Type::Str, Type::Str) => {
                        self.ast.push(Node::Operator {
                            op: Op::ConcatStr,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Str(a)), Some(Value::Str(b))) => {
                                    Some(vec![Value::Str(a.to_owned() + b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Str)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "-" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::SubInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Int(a - b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Int)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "*" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::MultInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Int(a * b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Int)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "/" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::DivInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Int(a / b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Int)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "%" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::ModInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Int(a % b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Int)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "==" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::EqInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Bool(a == b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Bool)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "<" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::LtInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Bool(a < b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Bool)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            ">" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::GtInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Bool(a > b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Bool)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "<=" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::LqInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Bool(a <= b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Bool)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            ">=" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Int, Type::Int) => {
                        self.ast.push(Node::Operator {
                            op: Op::GqInt,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Int(a)), Some(Value::Int(b))) => {
                                    Some(vec![Value::Bool(a >= b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Bool)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "og" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (Type::Bool, Type::Bool) => {
                        self.ast.push(Node::Operator {
                            op: Op::AndBool,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(Value::Bool(a)), Some(Value::Bool(b))) => {
                                    Some(vec![Value::Bool(*a && *b)])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(Type::Bool)
                    }
                    (_, _) => {
                        return Err(anyhow!(
                            "{} operator does not support {:?} and {:?}",
                            word,
                            a,
                            b
                        ))
                    }
                }
                Ok(true)
            }
            "dup" => {
                let Some(b) = self.type_stack.pop() else {
                    return Err(anyhow!("{} needs ateast 1 argument", word));
                };
                self.ast.push(Node::Operator {
                    op: Op::Dup,
                    arity: 1,
                    func: |args| Some(vec![args[0].clone(), args[0].clone()]),
                });
                self.type_stack.push(b);
                self.type_stack.push(b);
                Ok(true)
            }
            "slipp" => {
                let Some(b) = self.type_stack.pop() else {
                    return Err(anyhow!("{} needs ateast 1 argument", word));
                };
                self.ast.push(Node::Operator {
                    op: Op::Drop,
                    arity: 1,
                    func: |_args| None,
                });
                self.type_stack.push(b);
                self.type_stack.push(b);
                Ok(true)
            }
            "snu" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (_, _) => {
                        self.ast.push(Node::Operator {
                            op: Op::Swap,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(a), Some(b)) => {
                                    Some(vec![b.clone(), a.clone()])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(b);
                        self.type_stack.push(a);
                    }
                }
                Ok(true)
            }
            "over" => {
                let b = self.type_stack.pop().unwrap();
                let a = self.type_stack.pop().unwrap();
                match (a, b) {
                    (_, _) => {
                        self.ast.push(Node::Operator {
                            op: Op::Over,
                            arity: 2,
                            func: |args| match (args.get(0), args.get(1)) {
                                (Some(a), Some(b)) => {
                                    Some(vec![a.clone(), b.clone(), a.clone()])
                                }
                                _ => None,
                            },
                        });
                        self.type_stack.push(a);
                        self.type_stack.push(b);
                        self.type_stack.push(a);
                    }
                }
                Ok(true)
            }
            "skrivnl" => {
                let _ = self.type_stack.pop().unwrap();
                self.ast.push(Node::Operator {
                    op: Op::Println,
                    arity: 1,
                    func: |args| {
                        println!("{}", args[0]);
                        None
                    },
                });
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
