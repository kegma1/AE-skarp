use std::fmt;


#[derive(Debug)]
pub enum Op {
    SumInt,
    SubInt,
    MultInt,
    DivInt,
    ModInt,

    EqInt,
    LtInt,
    GtInt,
    LqInt,
    GqInt,

    Dup,
    Drop,

    Println,
}

pub struct Pointer {
    offset: isize,
}

impl Pointer {
    pub fn new(offset: isize) -> Pointer {
        Pointer { offset }
    }

    pub fn resolve(&self, current_pos: usize) -> usize {
        if self.offset >= 0 {
            current_pos + self.offset as usize
        } else {
            current_pos - (-self.offset) as usize
        }
    }
}

impl fmt::Display for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.offset)?;
        Ok(())
    }
}

pub enum Node {
    PushInt(i64),
    PushBool(bool),
    Operator {
        op: Op,
        arity: usize,
        func: fn(&[Value]) -> Option<Vec<Value>>,
    },
    Identifier(String),
    Jump(Pointer),
    JumpIfFalse(Pointer),
    If {
        condition: Vec<Node>,
        block: Vec<Node>,
    },
    Else {
        block: Vec<Node>,
    },
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::PushInt(x) => write!(f, "{}", x),
            Node::PushBool(x) => write!(f, "{}", x),
            Node::Operator { op, arity: _, func: _ } => write!(f, "{:?}", op),
            Node::Identifier(x) => write!(f, "{}", x),
            Node::If { condition, block } => {
                write!(f, "{:?}", condition)?;
                write!(f, "{:?}", block)
            },
            Node::Else { block } => write!(f, "{:?}", block),
            Node::Jump(x) => write!(f, "Jmp({})", x),
            Node::JumpIfFalse(x) => write!(f, "Jnt({})", x),
        }?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int,
    Bool,
}

pub struct Runtime {
    pub stack: Vec<Value>,
    pub op_counter: usize
}