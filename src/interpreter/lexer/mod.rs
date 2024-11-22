pub mod token;
use token::{Token, TokenType};

pub fn tokenize(source: &str) -> Result<Vec<Token>, ()> {
    let mut had_error = false;
    let mut tokens = Vec::new();
    let chars = source.chars().collect::<Vec<char>>();

    let mut line = 0;
    let mut i = 0;
    while i < chars.len() {
        let char = chars[i];
        match char {
            // handle the easy cases first
            '(' | ')' | '{' | '}' | '*' | '.' | ',' | '+' | '-' | ';' => {
                tokens.push(Token::new(
                    char.try_into().unwrap(),
                    char.to_string(),
                    line + 1,
                    i + 1,
                    1,
                ));
                i += 1;
            }
            // '=', '!', '<', and '>' can be single or double characters (e.g. '==' or '=')
            '=' | '<' | '>' | '!' => {
                let next_char = chars.get(i + 1);
                if let Some(&next_char) = next_char {
                    let (len, token_type) = match (char, next_char) {
                        ('=', '=') => (2, TokenType::EqualEqual),
                        ('!', '=') => (2, TokenType::BangEqual),
                        ('<', '=') => (2, TokenType::LessEqual),
                        ('>', '=') => (2, TokenType::GreaterEqual),
                        ('=', _) | ('!', _) | ('<', _) | ('>', _) => (1, char.try_into().unwrap()),
                        _ => {
                            eprintln!("[line {}] Error: Unexpected character", line + 1);
                            had_error = true;
                            (0, TokenType::EOF)
                        }
                    };
                    tokens.push(Token::new(
                        token_type,
                        chars[i..i + len].iter().collect(),
                        line + 1,
                        i + 1,
                        len,
                    ));
                    i += len;
                } else {
                    let token_type = char.try_into().unwrap();
                    tokens.push(Token::new(token_type, char.to_string(), line + 1, i + 1, 1));
                    i += 1;
                }
            }
            // slash needs to be handled separately because it can be a comment
            '/' => {
                let next_char = chars.get(i + 1);
                if let Some(&next_char) = next_char {
                    if next_char == '/' {
                        // just skip the rest of the line
                        while i < chars.len() && chars[i] != '\n' {
                            i += 1;
                        }
                        continue;
                    }
                    if next_char == '*' {
                        let mut nestings = 1;
                        i += 2;
                        while i < chars.len() && nestings > 0 {
                            let char = chars[i];
                            match (char, chars.get(i + 1)) {
                                ('*', Some('/')) => {
                                    nestings -= 1;
                                }
                                ('/', Some('*')) => {
                                    nestings += 1;
                                }
                                ('\n', _) => {
                                    line += 1;
                                }
                                _ => (),
                            }
                            i += 1;
                        }
                        // The closing "/"
                        i += 1;
                        continue;
                    }
                }
                tokens.push(Token::new(
                    TokenType::Slash,
                    char.to_string(),
                    line + 1,
                    i + 1,
                    1,
                ));
                i += 1;
            }
            // ignore whitespace
            ' ' | '\t' | '\r' => {
                i += 1;
            }
            // newlines need to be handled separately to keep track of the current line
            '\n' => {
                line += 1;
                i += 1;
            }
            // handle strings
            '"' => {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '"' {
                    if chars[j] == '\n' {
                        line += 1;
                    }
                    j += 1;
                }
                if j == chars.len() {
                    eprintln!("[line {}] Error: Unterminated string.", line + 1);
                    had_error = true;
                    break;
                }
                tokens.push(Token::new(
                    TokenType::String(chars[i + 1..j].iter().collect()),
                    chars[i..j + 1].iter().collect(),
                    line + 1,
                    i + 1,
                    j - i + 1,
                ));
                i = j + 1;
            }
            // handle numbers
            '0'..='9' => {
                let mut j = i + 1;
                while j < chars.len() && chars[j].is_ascii_digit() {
                    j += 1;
                }
                if j < chars.len()
                    && chars[j] == '.'
                    && chars.get(j + 1).map_or(false, |c| c.is_ascii_digit())
                {
                    j += 1;
                    while j < chars.len() && chars[j].is_ascii_digit() {
                        j += 1;
                    }
                }
                let lexeme = chars[i..j].iter().collect::<String>();
                let parsed_number = lexeme.parse::<f64>().unwrap();
                tokens.push(Token::new(
                    TokenType::Number(parsed_number),
                    lexeme.to_string(),
                    line + 1,
                    i + 1,
                    j - i,
                ));
                i = j;
            }
            char => {
                // handle identifiers
                if char.is_alphabetic() || char == '_' {
                    let mut j = i + 1;
                    while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '_') {
                        j += 1;
                    }
                    let lexeme = chars[i..j].iter().collect::<String>();
                    let token_type = lexeme
                        .as_str()
                        .try_into()
                        .unwrap_or(TokenType::Identifier(lexeme.clone()));
                    tokens.push(Token::new(token_type, lexeme, line + 1, i + 1, j - i));
                    i = j;
                } else {
                    eprintln!("[line {}] Error: Unexpected character: {}", line + 1, char);
                    had_error = true;
                    i += 1;
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::EOF, String::new(), 0, 0, 0));

    if had_error {
        Err(())
    } else {
        Ok(tokens)
    }
}
