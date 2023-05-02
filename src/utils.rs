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

    Println,
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
    If {
        condition: Vec<Node>,
        block: Vec<Node>,
    },
    Else {
        block: Vec<Node>,
    },
    While {
        condition: Vec<Node>,
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
            Node::While { condition, block } => {
                write!(f, "{:?}", condition)?;
                write!(f, "{:?}", block)
            },
        }?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
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