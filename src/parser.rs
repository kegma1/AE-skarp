use crate::common::Type;
use crate::lexer::Lexem;

#[derive(Debug, PartialEq)]
pub enum Op {

    AddInt,
    SubInt,
    MultInt,
    DivInt,
    ModInt,

    EqInt,
    LtInt,

    Duplicate,

    Println,
}

#[derive(Debug)]
pub enum Node {
    PushInt(i64),
    PushBool(bool),

    Operator(Op),

    If{condition: Vec<Node>, block: Vec<Node>}
}

pub fn parse(src: Vec<Lexem>, opt_type_stack: Option<&Vec<Type>>) -> Result<(Vec<Node>, Vec<Type>), &'static str> {
    let mut ast: Vec<Node> = vec![];

    let mut type_stack: Vec<Type> = vec![];
    if opt_type_stack.is_some() {
        type_stack = opt_type_stack.unwrap().to_vec()
    }

    // let mut jump_stack: Vec<usize> = vec![];
    // let mut keyword_stack: Vec<Keyword> = vec![];

    let mut src_iter = src.iter().peekable();
    while let Some(lexem) = src_iter.next() {
        match lexem {
            Lexem::Identifier(ident) => match ident.as_str() {
                "skrivnl" => {
                    ast.push(Node::Operator(Op::Println));
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
                            ast.push(Node::Operator(Op::AddInt));
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
                            ast.push(Node::Operator(Op::SubInt));
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
                            ast.push(Node::Operator(Op::MultInt));
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
                            ast.push(Node::Operator(Op::DivInt));
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
                            ast.push(Node::Operator(Op::ModInt));
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
                            ast.push(Node::Operator(Op::EqInt));
                            type_stack.push(Type::Bool)
                        }
                        (_, _) => return Err("ERROR: invalid types for eq operator"),
                    }
                }
                "<" => {
                    let type_b = type_stack.pop().unwrap();
                    let type_a = type_stack.pop().unwrap();

                    match (type_a, type_b) {
                        (Type::Int, Type::Int) => {
                            ast.push(Node::Operator(Op::LtInt));
                            type_stack.push(Type::Bool)
                        }
                        (_, _) => return Err("ERROR: invalid types for eq operator"),
                    }
                }
                "dup" =>  {
                    let type_b = type_stack.pop().unwrap();

                    ast.push(Node::Operator(Op::Duplicate));

                    type_stack.push(type_b);
                    type_stack.push(type_b)
                }
                _ => return Err("ERROR: Unsupported operator"),
            },
            Lexem::Literal((value, typ)) => match typ {
                Type::Int => {
                    ast.push(Node::PushInt(value.parse().expect("Should not happen")));
                    type_stack.push(typ.clone())
                }
                Type::Bool => {
                    if value == "sann" {
                        ast.push(Node::PushBool(true));
                        type_stack.push(typ.clone())
                    } else {
                        ast.push(Node::PushBool(false));
                        type_stack.push(typ.clone())
                    }
                }
                _ => return Err("ERROR: Unsupported type"),
            },
            Lexem::Keyword(word) => match word.as_str() {
                "hvis" => {
                    let mut condition: Vec<Lexem> = vec![];
                    while src_iter.peek() != Some(&&Lexem::Separator(":".to_owned())) {
                        condition.push(src_iter.next().unwrap().clone())
                    }
                    let _ = src_iter.next();
                    let mut block: Vec<Lexem> = vec![];
                    if let Some(Lexem::Indent) = src_iter.next() {
                        while src_iter.peek() != Some(&&Lexem::Dedent) {
                            block.push(src_iter.next().unwrap().clone())
                        }
                    } else {
                        return Err("invalid syntax, expected an indent");
                    }
                    let (condition_ast, mut condition_stack) = parse(condition, Some(&type_stack)).unwrap();
                    let (block_ast, block_stack) = parse(block, Some(&type_stack)).unwrap();

                    if let Some(Type::Bool) = condition_stack.pop() {
                        ast.push(Node::If { 
                            condition: condition_ast, 
                            block: block_ast 
                        });

                        type_stack =  block_stack
                    } else {
                        return Err("condition dose not result in a bool");
                    }


                },
                _ => return Err("ERROR: Unsupported keyword"),
            },
            Lexem::Separator(_) => (),
            Lexem::Indent => (),
            Lexem::Dedent => ()
        }
    }
    Ok((ast, type_stack))
}
