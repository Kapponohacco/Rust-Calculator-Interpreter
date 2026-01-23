use crate::engine::CalcError;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens{
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    LParen,
    RParen,
    Var(String),
    Assign,
}

pub fn tokenize(input: &str) -> Result<Vec<Tokens>, CalcError> {
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch{
            '0'..='9'|'.' => {
                let mut acc = String::new();
                if (ch == '.') && (acc.is_empty()){
                    acc.push_str("0.");
                    chars.next();
                }
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        acc.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let value = acc.parse::<f64>()
                    .map_err(|_|{CalcError::InvalidExpression((acc))});
                tokens.push(Tokens::Number(value?));
            }
            '+' => {
                chars.next();
                tokens.push(Tokens::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Tokens::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Tokens::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Tokens::Slash);
            }
            '(' => {
                chars.next();
                tokens.push(Tokens::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Tokens::RParen);
            }
            '=' => {
                chars.next();
                tokens.push(Tokens::Assign);
            }
            '^' =>  {
                chars.next();
                tokens.push(Tokens::Power);
            }
            c if c.is_ascii_whitespace() => {
                chars.next();
            }
            c if c.is_ascii_alphanumeric() => {
                let mut acc = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || c == '_' {
                        acc.push(ch);
                        chars.next();
                    }
                    else{
                        break;
                    }
                }
                tokens.push(Tokens::Var(acc));
            }
            _ => {
                return Err(CalcError::InvalidExpression(ch.to_string()));
            }


        }
    }
    Ok(tokens)
}