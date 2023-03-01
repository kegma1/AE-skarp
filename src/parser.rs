use crate::common::Type;
use crate::lexer::Lexem;

#[derive(Debug, PartialEq)]
pub enum Op {
    PushInt(i64),

    AddInt,
    SubInt,
    MultInt,
    DivInt,
    ModInt,

    EqInt,

    Println,

    JumpIfFalse(Option<usize>),
    Jump(Option<usize>),

    StartBlock,
    EndBlock,
}

#[derive(Debug)]
enum Keyword {
    If,
    Else,
}

pub fn parse(src: Vec<Lexem>) -> Result<Vec<Op>, &'static str> {
    let mut ast: Vec<Op> = vec![];

    let mut type_stack: Vec<Type> = vec![];
    let mut jump_stack: Vec<usize> = vec![];
    let mut keyword_stack: Vec<Keyword> = vec![];

    let mut src_iter = src.iter();
    while let Some(lexem) = src_iter.next() {
        match lexem {
            Lexem::Identifier(ident) => match ident.as_str() {
                "skrivnl" => {
                    ast.push(Op::Println);
                    type_stack.pop();
                }
                _ => return Err("ERROR: unknown word"),
            },
            Lexem::Operator(op) => match op.as_str() {
                "+" => {
                    let type_b = type_stack.pop().unwrap();
                    let type_a = type_stack.pop().unwrap();

                    match (type_a, type_b) {
                        (Type::Int, Type::Int) => {
                            ast.push(Op::AddInt);
                            type_stack.push(Type::Int)
                        }
                        (_, _) => return Err("ERROR: invalid types for add operator"),
                    }
                }
                "-" => {
                    let type_b = type_stack.pop().unwrap();
                    let type_a = type_stack.pop().unwrap();

                    match (type_a, type_b) {
                        (Type::Int, Type::Int) => {
                            ast.push(Op::SubInt);
                            type_stack.push(Type::Int)
                        }
                        (_, _) => return Err("ERROR: invalid types for sub operator"),
                    }
                }
                "*" => {
                    let type_b = type_stack.pop().unwrap();
                    let type_a = type_stack.pop().unwrap();

                    match (type_a, type_b) {
                        (Type::Int, Type::Int) => {
                            ast.push(Op::MultInt);
                            type_stack.push(Type::Int)
                        }
                        (_, _) => return Err("ERROR: invalid types for mult operator"),
                    }
                }
                "/" => {
                    let type_b = type_stack.pop().unwrap();
                    let type_a = type_stack.pop().unwrap();

                    match (type_a, type_b) {
                        (Type::Int, Type::Int) => {
                            ast.push(Op::DivInt);
                            type_stack.push(Type::Int)
                        }
                        (_, _) => return Err("ERROR: invalid types for div operator"),
                    }
                }
                "%" => {
                    let type_b = type_stack.pop().unwrap();
                    let type_a = type_stack.pop().unwrap();

                    match (type_a, type_b) {
                        (Type::Int, Type::Int) => {
                            ast.push(Op::ModInt);
                            type_stack.push(Type::Int)
                        }
                        (_, _) => return Err("ERROR: invalid types for mod operator"),
                    }
                }
                "==" => {
                    let type_b = type_stack.pop().unwrap();
                    let type_a = type_stack.pop().unwrap();

                    match (type_a, type_b) {
                        (Type::Int, Type::Int) => {
                            ast.push(Op::EqInt);
                            type_stack.push(Type::Bool)
                        }
                        (_, _) => return Err("ERROR: invalid types for eq operator"),
                    }
                }
                _ => return Err("ERROR: Unsupported operator"),
            },
            Lexem::Literal((value, typ)) => match typ {
                Type::Int => {
                    ast.push(Op::PushInt(value.parse().expect("Should not happen")));
                    type_stack.push(typ.clone())
                }
                _ => return Err("ERROR: Unsupported type"),
            },
            Lexem::Keyword(word) => match word.as_str() {
                "hvis" => keyword_stack.push(Keyword::If),
                "ellers" => keyword_stack.push(Keyword::Else),
                _ => return Err("ERROR: Unsupported keyword"),
            },
            Lexem::Separator(sep) => match sep.as_str() {
                ":" => {
                    let kw = keyword_stack.pop().unwrap();
                    match kw {
                        Keyword::If => {
                            let type_condition = type_stack.pop().unwrap();

                            if type_condition == Type::Bool {
                                ast.push(Op::JumpIfFalse(None));

                                jump_stack.push(ast.len() - 1);
                            }
                        }
                        Keyword::Else => {
                            let possible_if = jump_stack.pop().unwrap();
                            if ast[possible_if] == Op::JumpIfFalse(None) {
                                ast[possible_if] = Op::JumpIfFalse(Some(ast.len() - 1))
                            }
                        },
                    }
                }

                _ => return Err("ERROR: Unsupported seperator"),
            },
            Lexem::Indent => ast.push(Op::StartBlock),
            Lexem::Dedent => ast.push(Op::EndBlock),
        }
    }

    Ok(ast)
}
