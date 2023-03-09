use crate::parser::Op;

#[derive(Clone)]
enum Value {
    Int(i64),
    Bool(bool)
}

pub fn interpret(ast: Vec<Op>) -> Result<u8, &'static str> {
    let mut stack: Vec<Value> = vec![];

    let mut op_ptr = 0;
    let mut _scope_num = 0;
    while let Some(op) = ast.get(op_ptr) {
        match op {
            Op::PushInt(x) => stack.push(Value::Int(*x)),
            Op::PushBool(x) => stack.push(Value::Bool(*x)),
            Op::AddInt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match (a, b) {
                    (Value::Int(val_a), Value::Int(val_b)) => {
                        stack.push(Value::Int(val_a + val_b))
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::SubInt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match (a, b) {
                    (Value::Int(val_a), Value::Int(val_b)) => {
                        stack.push(Value::Int(val_a - val_b))
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::MultInt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match (a, b) {
                    (Value::Int(val_a), Value::Int(val_b)) => {
                        stack.push(Value::Int(val_a * val_b))
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::DivInt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match (a, b) {
                    (Value::Int(val_a), Value::Int(val_b)) => {
                        stack.push(Value::Int(val_a / val_b))
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::ModInt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match (a, b) {
                    (Value::Int(val_a), Value::Int(val_b)) => {
                        stack.push(Value::Int(val_a % val_b))
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::EqInt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match (a, b) {
                    (Value::Int(val_a), Value::Int(val_b)) => {
                        stack.push(Value::Bool(val_a == val_b))
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::LtInt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match (a, b) {
                    (Value::Int(val_a), Value::Int(val_b)) => {
                        stack.push(Value::Bool(val_a < val_b))
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::Duplicate => {
                let b = stack.pop().unwrap();
                stack.push(b.clone());
                stack.push(b.clone());
            },
            Op::Println => {
                let b = stack.pop().unwrap();
                
                match b {
                    Value::Int(val_b) => {
                        println!("{}", val_b)
                    }
                    Value::Bool(val_b) => {
                        if val_b {
                            println!("sann")
                        } else {
                            println!("usann")
                        }
                    }
                }
            },
            Op::JumpIfFalse(addr) => {
                let b = stack.pop().unwrap();
                
                match b {
                    Value::Bool(val_b) => {
                        if !val_b {
                            op_ptr = addr.unwrap()
                        }
                    }
                    _ => return  Err("ERROR: Should not happen")
                }
            },
            Op::Jump(addr) => {
                op_ptr = addr.unwrap()
            },
            Op::StartBlock => _scope_num += 1,
            Op::EndBlock => _scope_num -= 1,
        }
        op_ptr += 1;
    }
    Ok(0)
}
