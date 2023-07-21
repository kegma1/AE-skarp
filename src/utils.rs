use std::{fmt, collections::HashMap};

#[derive(Debug)]
pub enum Op {
    SumInt,
    SubInt,
    MultInt,
    DivInt,
    ModInt,

    ConcatStr,

    EqInt,
    LtInt,
    GtInt,
    LqInt,
    GqInt,
    AndBool,

    Dup,
    Drop,
    Swap,
    Over,

    Println,
}

pub struct JumpPointer {
    offset: isize,
}

impl JumpPointer {
    pub fn new(offset: isize) -> JumpPointer {
        JumpPointer { offset }
    }

    pub fn resolve(&self, current_pos: usize) -> usize {
        if self.offset >= 0 {
            current_pos + self.offset as usize
        } else {
            current_pos - (-self.offset) as usize
        }
    }
}

impl fmt::Display for JumpPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.offset)?;
        Ok(())
    }
}

pub enum Node {
    PushInt(i64),
    PushBool(bool),
    PushStr(String),
    Operator {
        op: Op,
        arity: usize,
        func: fn(&[Value]) -> Option<Vec<Value>>,
    },
    Identifier(String),
    Jump(JumpPointer),
    JumpIfFalse(JumpPointer),
    EndOfIf,
    DefineConst(String),
    Return(String),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::PushInt(x) => write!(f, "{}", x),
            Node::PushBool(x) => write!(f, "{}", x),
            Node::PushStr(x) => write!(f, "\"{}\"", x),
            Node::Operator {
                op,
                arity: _,
                func: _,
            } => write!(f, "{:?}", op),
            Node::Identifier(x) => write!(f, "{}", x),
            Node::Jump(x) => write!(f, "Jmp({})", x),
            Node::JumpIfFalse(x) => write!(f, "Jnt({})", x),
            Node::EndOfIf => write!(f, "EndOfIf"),
            Node::DefineConst(x) => write!(f, "DefConst({})", x),
            Node::Return(_) =>write!(f, "Ret"),
        }?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Int(i64),
    Bool(bool),
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Bool(b) => {
                if *b {
                    write!(f, "sann")
                } else {
                    write!(f, "usann")
                }
            },
            Value::Str(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Str
}

pub struct Runtime {
    pub stack: Vec<Value>,
    pub mem: HashMap<String, Value>,
    pub op_counter: usize,
}
