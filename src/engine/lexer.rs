use crate::engine::CalcError;

/*pub enum Tokens{
    Number(f64),
    Op(String),
    LParen,
    RParen,
    Var(String),
    Func(String) <--- for basic functions like sin, cos. Will think about the implementation later.
*/
#[derive(Debug, Clone, PartialEq)]
pub enum Token{ //will think about changing the Tokens for operators to one token with string like above
    Number(f64),
    Plus,
    PlusEqual,
    Minus,
    UnaryMinus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Power,
    LParen,
    RParen,
    Var(String),
    Equal,
    Assign,
    EndOfFile,
}

fn push_op(chars: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>, normal: Token, compound: Token, ) {
    chars.next();
    if chars.peek() == Some(&'=') {
        chars.next();
        tokens.push(compound);
    } else {
        tokens.push(normal);
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens: Vec<Token> = Vec::new();
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
                    .map_err(|_|{CalcError::InvalidExpression(acc)});
                tokens.push(Token::Number(value?));
            },
            '+' => push_op(&mut chars, &mut tokens, Token::Plus, Token::PlusEqual),
            '-' => push_op(&mut chars, &mut tokens, Token::Minus, Token::MinusEqual),
            '*' => push_op(&mut chars, &mut tokens, Token::Star, Token::StarEqual),
            '/' => push_op(&mut chars, &mut tokens, Token::Slash, Token::SlashEqual),
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            },
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            },
            '=' => push_op(&mut chars, &mut tokens, Token::Assign, Token::Equal),

            '^' =>  {
                chars.next();
                tokens.push(Token::Power);
            },
            c if c.is_ascii_whitespace() => {
                chars.next();
            },
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
                tokens.push(Token::Var(acc));
            },
            ';' => {
                chars.next();
                tokens.push(Token::EndOfFile);
            }
            _ => {
                return Err(CalcError::InvalidExpression(ch.to_string()));
            }
        }
    }
    if tokens.is_empty() {
        Err(CalcError::EmptyExpression)
    }
    else {
        Ok(tokens)
    }
}