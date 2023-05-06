use crate::utils::*;
use anyhow::{anyhow, Result};

pub fn eval(ast: Vec<Node>) -> Result<u8> {
    let mut rt = Runtime{ stack:vec![], op_counter: 0 };

    while let Some(node) =  ast.get(rt.op_counter) {
        // println!("Stack: {:?}\nNode: {:?}", rt.stack, node);

        match node {
            Node::PushInt(x) => rt.stack.push(Value::Int(*x)),
            Node::PushBool(x) => rt.stack.push(Value::Bool(*x)),
            Node::Operator { op: _, arity, func } => {
                let mut args = vec![];
                for _ in 1..=*arity {
                    args.push(rt.stack.pop().expect("should not happen"))
                }
                args.reverse();
                let opt_res = func(&args);
                match  opt_res {
                    Some(mut res) => rt.stack.append(&mut res),
                    None => (),
                }
                
            },
            Node::Identifier(_) => todo!(),
            Node::If { condition: _, block: _ } => todo!(),
            Node::Else { block: _ } => todo!(),
            Node::Jump(x) => rt.op_counter = x.resolve(rt.op_counter),
            Node::JumpIfFalse(x) => {
                let Some(Value::Bool(condition_resualt)) = rt.stack.pop() else {
                    return Err(anyhow!("have not yet made typechecking for loops"))
                };

                if !condition_resualt {
                    rt.op_counter = x.resolve(rt.op_counter)
                }
            },
        }
        rt.op_counter += 1;
    }
    Ok(0u8)
}