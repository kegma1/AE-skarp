#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    String,
    Int,
    Float,
    Bool,
}

pub const KEYWORDS: [&str; 3] = ["hvis", "ellers", "når"];

pub const OPERATOR: [&str; 13] = [
    "+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=", "=", "dup",
];

pub const SEPERATORS: [&str; 4] = [":", ";", "[", "]"];
