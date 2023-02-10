#[derive(Debug)]
pub enum Lexem {
    Identifier(String),
    Operator(String),
    Literal(String),
    Keyword(String),
    _Separator(String),
    Indent,
}

const KEYWORDS: [&str; 3] = ["funk", "konst", "var"];

const OPERATOR: [&str; 5] = ["+", "-", "*", "/", "%"];

const _SEPERATORS: [char; 3] = [':', '(', ')'];

pub fn lex(src: &String) -> Result<Vec<Lexem>, &'static str> {
    let mut current_word = "".to_string();
    let mut lexed_src: Vec<Lexem> = vec![];
    let mut i = 0;
    while i < src.len() {
        let c = src.chars().nth(i).unwrap();
        match c {
            '"' => {
                let (str, offset) = scan_string(&src[i + 1..src.len()]).unwrap();
                lexed_src.push(Lexem::Literal(str));
                i += offset
            }
            _ if c.is_whitespace() => {
                println!("'{}'", current_word);
                if current_word == "    " || current_word == "\t" {
                    lexed_src.push(Lexem::Indent);
                } else if KEYWORDS.contains(&current_word.as_ref()) {
                    lexed_src.push(Lexem::Keyword(current_word));
                } else if OPERATOR.contains(&current_word.as_ref()) {
                    lexed_src.push(Lexem::Operator(current_word));
                } else {
                    if current_word != "" {
                        lexed_src.push(Lexem::Identifier(current_word));
                    }
                }
                current_word = "".to_string();
            }
            _ => current_word.push(c),
        }

        i += 1
    }

    Ok(lexed_src)
}

fn scan_string(str: &str) -> Option<(String, usize)> {
    let mut scanned_str = "".to_string();
    let mut i_offset = 0;

    for c in str.chars() {
        if c != '"' {
            scanned_str.push(c);
            i_offset += 1;
        } else {
            break;
        }
    }

    Some((scanned_str.clone(), i_offset + 1))
}
