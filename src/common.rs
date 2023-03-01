#[derive(Debug)]
pub enum Type {
    String,
    Int,
    Float,
    Bool,
}

pub const KEYWORDS: [&str; 3] = ["funk", "konst", "var"];

pub const OPERATOR: [&str; 5] = ["+", "-", "*", "/", "%"];

pub const SEPERATORS: [&str; 4] = [":", ";", "[", "]"];