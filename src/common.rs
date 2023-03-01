#[derive(Debug, Clone, Copy)]
pub enum Type {
    String,
    Int,
    Float,
    Bool,
}

pub const KEYWORDS: [&str; 3] = ["hvis", "ellvis", "ellers"];

pub const OPERATOR: [&str; 11] = ["+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">="];

pub const SEPERATORS: [&str; 5] = [":", ";", "[", "]", "="];
