use crate::utils::*;
use anyhow::{anyhow, Result};

pub fn eval(ast: Vec<Node>) -> Result<u8> {
    let mut rt = Runtime{ stack:vec![] };

    for node in ast {
        println!("{:?}", rt.stack);

        match node {
            Node::PushInt(x) => rt.stack.push(Value::Int(x)),
            Node::PushBool(x) => rt.stack.push(Value::Bool(x)),
            Node::Operator { op: _, arity, func } => {
                let mut args = vec![];
                for _ in 1..=arity {
                    args.push(rt.stack.pop().expect("should not happen"))
                }
                args.reverse();
                let Some(mut res) = func(&args) else {
                    continue;                    
                };
                rt.stack.append(&mut res)
            },
            Node::Identifier(_) => todo!(),
            Node::If { condition, block } => todo!(),
            Node::Else { block } => todo!(),
            Node::While { condition, block } => todo!(),
        }
    }
    Ok(0u8)
}