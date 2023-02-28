#[derive(Debug)]
pub enum Lexem {
    Identifier(String),
    Operator(String),
    Literal(String),
    String(String),
    Keyword(String),
    Separator(String),
    Indent,
    Dedent,
}

const KEYWORDS: [&str; 3] = ["funk", "konst", "var"];

const OPERATOR: [&str; 5] = ["+", "-", "*", "/", "%"];

const SEPERATORS: [&str; 4] = [":", ";", "[", "]"];

pub fn lex(src: &String) -> Result<Vec<Lexem>, &'static str> {
    let mut lexed_src: Vec<Lexem> = vec![];
    let mut current_indentation = 0;

    let mut lines = src.lines().peekable();

    while let Some(line) = lines.next() {
        let mut line_indentation = 0;

        for c in line.chars() {
            match c {
                ' ' => line_indentation += 1,
                '\t' => line_indentation += 4,
                _ => break,
            }
        }

        if line_indentation > current_indentation {
            lexed_src.push(Lexem::Indent);
            current_indentation = line_indentation;
        } else if line_indentation < current_indentation {
            while line_indentation < current_indentation {
                lexed_src.push(Lexem::Dedent);
                current_indentation -= 4;
            }
        }

        lexed_src.append(&mut lex_line(line))
    }

    while current_indentation > 0 {
        lexed_src.push(Lexem::Dedent);
        current_indentation -= 4;
    }

    Ok(lexed_src)
}

fn lex_line(line: &str) -> Vec<Lexem> {
    let mut tokens = vec![];
    let mut current_token = String::new();

    //modes
    let mut in_string = false;

    for c in line.chars() {
        if c.is_whitespace() {
            if in_string {
                current_token.push(c);
            } else if !current_token.is_empty() {
                tokens.push(get_token(&current_token));
                current_token.clear()
            }
        } else if SEPERATORS.contains(&c.to_string().as_str()){
            if in_string {
                current_token.push(c);
            } else if !current_token.is_empty() {
                tokens.push(get_token(&current_token));
                tokens.push(Lexem::Separator(c.to_string()));
                current_token.clear()
            } else {
                tokens.push(Lexem::Separator(c.to_string()));
                current_token.clear()
            }
        } else if c == '"' {
            if in_string {
                tokens.push(Lexem::String(current_token.to_string()));
                current_token.clear();
            } else {
                if !current_token.is_empty() {
                    tokens.push(get_token(&current_token));
                    current_token.clear();
                }
            }
            in_string = !in_string;
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        tokens.push(get_token(&current_token));
    }

    tokens
}

fn get_token(word: &String) -> Lexem {
    if KEYWORDS.contains(&word.as_str()) {
        Lexem::Keyword(word.to_string())
    } else if OPERATOR.contains(&word.as_str()) {
        Lexem::Operator(word.to_string())
    } else if SEPERATORS.contains(&word.as_str()) {
        Lexem::Separator(word.to_string())
    } else if word == "sann" {
        Lexem::Literal("sann".to_string())
    } else if word == "usann" {
        Lexem::Literal("usann".to_string())
    } else if word.parse::<i64>().is_ok() {
        Lexem::Literal(word.to_string())
    } else if match is_float(&word) {
        Some(x) => x,
        None => false,
    } {
        Lexem::Literal(word.to_string())
    } else {
        Lexem::Identifier(word.to_string())
    }
}

fn is_float(s: &str) -> Option<bool> {
    if s == "iet" {
        return Some(true);
    } else if s == "uendelig" || s == "-uendelig" {
        Some(true)
    } else if let Ok(_) = s.parse::<f64>() {
        println!("skriv på norsk for svarte helvete");
        None
    } else if let Ok(num) = s.replace(",", ".").parse::<f64>() {
        Some(num.is_finite())
    } else {
        Some(false)
    }
}
