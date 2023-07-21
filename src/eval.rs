use std::collections::HashMap;

use crate::utils::*;
use anyhow::{anyhow, Result};

pub fn eval(ast: Vec<Node>) -> Result<u8> {
    let mut rt = Runtime {
        stack: vec![],
        mem: HashMap::new(),
        op_counter: 0,
    };

    while let Some(node) = ast.get(rt.op_counter) {
        // println!("Stack: {:?}\nNode: {:?}", rt.stack, node);

        match node {
            Node::PushInt(x) => rt.stack.push(Value::Int(*x)),
            Node::PushBool(x) => rt.stack.push(Value::Bool(*x)),
            Node::PushStr(x) => rt.stack.push(Value::Str(x.clone())),
            Node::Operator { op: _, arity, func } => {
                let mut args = vec![];
                for _ in 1..=*arity {
                    args.push(rt.stack.pop().expect("should not happen"))
                }
                args.reverse();
                let opt_res = func(&args);
                match opt_res {
                    Some(mut res) => rt.stack.append(&mut res),
                    None => (),
                }
            }
            Node::Identifier(name) => {
                let optional_value = rt.mem.get(name);
                if let Some(value) = optional_value {
                    rt.stack.push(value.clone());
                } else {
                    return Err(anyhow!("could not find identifier '{}'", name));
                }
            },
            Node::Jump(x) => rt.op_counter = x.resolve(rt.op_counter),
            Node::JumpIfFalse(x) => {
                let Some(Value::Bool(condition_resualt)) = rt.stack.pop() else {
                    return Err(anyhow!("have not yet made typechecking for loops"))
                };

                if !condition_resualt {
                    rt.op_counter = x.resolve(rt.op_counter)
                }
            }
            Node::EndOfIf => (),
            Node::DefineConst(name) => {
                rt.mem.insert(name.to_string(), Value::Null);
            },
            Node::Return(name) => {
                let value = rt.stack.pop().unwrap();
                rt.mem.insert(name.to_string(), value);
            },
        }
        rt.op_counter += 1;
    }
    Ok(0u8)
}
