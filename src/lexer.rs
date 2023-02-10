#[derive(Debug)]
pub enum Lexem {
    Identifier(String),
    Operator(String),
    Literal(String),
    Keyword(String),
    Separator(String),
    Indent
}

pub fn lex(src: &String) -> Result<Vec<Lexem>, &'static str> {
    let mut current_word = "".to_string();
    let mut lexed_src: Vec<Lexem> = vec![];
    for c in src.chars() {
        current_word.push(c);
        
        match current_word.as_str() {
            "    " => {
                lexed_src.push(Lexem::Indent);
                current_word = "".to_string()
            }
            _ => ()
        }
    }

    Ok(lexed_src)
}