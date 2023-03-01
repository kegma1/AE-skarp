#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    String,
    Int,
    Float,
    Bool,
}

pub const KEYWORDS: [&str; 3] = ["hvis", "ellvis", "ellers"];

pub const OPERATOR: [&str; 12] = [
    "+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=", "=",
];

pub const SEPERATORS: [&str; 4] = [":", ";", "[", "]"];
