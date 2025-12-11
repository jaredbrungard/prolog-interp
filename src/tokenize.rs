use crate::types::Token;

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            c if c.is_ascii_uppercase() || c == '_' => {
                let mut var_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        var_str.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Var(var_str));
            }
            c if c.is_ascii_lowercase() => {
                let mut atom_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        atom_str.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Atom(atom_str));
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '.' => {
                tokens.push(Token::Period);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ':' => {
                chars.next();
                if let Some(&'-') = chars.peek() {
                    chars.next();
                    tokens.push(Token::ColonHyphen);
                } else {
                    return Err("unexpected character: ':'".to_string());
                }
            }
            c if c.is_whitespace() => {
                chars.next();
            }
            _ => {
                return Err(format!("unexpected character: '{}'", ch));
            }
        }
    }

    Ok(tokens)
}
