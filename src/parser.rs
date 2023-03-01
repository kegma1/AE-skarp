use crate::common::Type;
use crate::lexer::Lexem;

#[derive(Debug)]
pub enum Op {
    PushInt(i64),
    AddInt,
    Println,
}

pub fn parse(src: Vec<Lexem>) -> Result<Vec<Op>, &'static str> {
    let mut ast: Vec<Op>  = vec![];
    let mut type_stack: Vec<Type> = vec![];
    for lexem in src {
        match lexem {
            Lexem::Identifier(ident) => {
                match ident.as_str() {
                    "skrivnl" => {
                        ast.push(Op::Println);
                        type_stack.pop();
                    }
                    _ => return Err("ERROR: unknown word")
                }
            },
            Lexem::Operator(op) => {
                match op.as_str() {
                    "+" => {
                        let type_b = type_stack.pop().unwrap();
                        let type_a = type_stack.pop().unwrap();
                        
                        match (type_a, type_b) {
                            (Type::Int, Type::Int) => {
                                ast.push(Op::AddInt);
                                type_stack.push(Type::Int)
                            }
                            (_, _) => return Err("ERROR: invalid types for add operator")
                        }
                    },
                    _ => return Err("ERROR: Unsupported operator"),
                }
            },
            Lexem::Literal((value, typ)) => {
                match typ {
                    Type::Int => {
                        ast.push(Op::PushInt(value.parse().expect("Should not happen")));
                        type_stack.push(typ)
                    },
                    _ => return Err("ERROR: Unsupported type"),
                }
            },
            Lexem::Keyword(_) => todo!(),
            Lexem::Separator(_) => todo!(),
            Lexem::Indent => todo!(),
            Lexem::Dedent => todo!(),
        }
    }

    Ok(ast)
}

// fn infer_type(type_stack: Vec<Type>, arg_count: usize) -> Type {

// }