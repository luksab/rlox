pub mod token;
use token::{Token, TokenType};

pub struct LexerInstance {
    pub had_error: bool,
    pub tokens: Vec<Token>,
}

impl LexerInstance {
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = true;
    }

    pub fn new() -> Self {
        LexerInstance {
            had_error: false,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self, source: &str) {
        let mut tokens = Vec::new();
        let chars = source.chars().collect::<Vec<char>>();

        let mut line = 0;
        let mut i = 0;
        while i < chars.len() {
            let char = chars[i];
            match char {
                // handle the easy cases first
                '(' | ')' | '{' | '}' | '*' | '.' | ',' | '+' | '-' | ';' => {
                    tokens.push(Token {
                        inner: match char {
                            '(' => TokenType::LeftParen,
                            ')' => TokenType::RightParen,
                            '{' => TokenType::LeftBrace,
                            '}' => TokenType::RightBrace,
                            '*' => TokenType::Star,
                            '.' => TokenType::Dot,
                            ',' => TokenType::Comma,
                            '+' => TokenType::Plus,
                            '-' => TokenType::Minus,
                            ';' => TokenType::Semicolon,
                            _ => unreachable!(),
                        },
                        lexeme: char.to_string(),
                        line: line + 1,
                        start_column: i + 1,
                        length: 1,
                    });
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
                            ('=', _) => (1, TokenType::Equal),
                            ('!', _) => (1, TokenType::Bang),
                            ('<', _) => (1, TokenType::Less),
                            ('>', _) => (1, TokenType::Greater),
                            _ => {
                                self.error(line + 1, "Unexpected character");
                                (0, TokenType::EOF)
                            }
                        };
                        tokens.push(Token {
                            inner: token_type,
                            lexeme: chars[i..i + len].iter().collect(),
                            line: line + 1,
                            start_column: i + 1,
                            length: len,
                        });
                        i += len;
                    } else {
                        let token_type = match char {
                            '=' => TokenType::Equal,
                            '!' => TokenType::Bang,
                            '<' => TokenType::Less,
                            '>' => TokenType::Greater,
                            _ => unreachable!(),
                        };
                        tokens.push(Token {
                            inner: token_type,
                            lexeme: char.to_string(),
                            line: line + 1,
                            start_column: i + 1,
                            length: 1,
                        });
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
                    tokens.push(Token {
                        inner: TokenType::Slash,
                        lexeme: char.to_string(),
                        line: line + 1,
                        start_column: i + 1,
                        length: 1,
                    });
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
                        self.error(line + 1, "Unterminated string.");
                        break;
                    }
                    tokens.push(Token {
                        inner: TokenType::String(chars[i + 1..j].iter().collect()),
                        lexeme: chars[i..j + 1].iter().collect(),
                        line: line + 1,
                        start_column: i + 1,
                        length: j - i + 1,
                    });
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
                    tokens.push(Token {
                        inner: TokenType::Number(parsed_number),
                        lexeme: lexeme.to_string(),
                        line: line + 1,
                        start_column: i + 1,
                        length: j - i,
                    });
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
                        let token_type = match lexeme.as_str() {
                            "and" => TokenType::And,
                            "class" => TokenType::Class,
                            "else" => TokenType::Else,
                            "false" => TokenType::False,
                            "for" => TokenType::For,
                            "fun" => TokenType::Fun,
                            "if" => TokenType::If,
                            "nil" => TokenType::Nil,
                            "or" => TokenType::Or,
                            "print" => TokenType::Print,
                            "return" => TokenType::Return,
                            "super" => TokenType::Super,
                            "this" => TokenType::This,
                            "true" => TokenType::True,
                            "var" => TokenType::Var,
                            "while" => TokenType::While,
                            _ => TokenType::Identifier(lexeme.clone()),
                        };
                        tokens.push(Token {
                            inner: token_type,
                            lexeme,
                            line: line + 1,
                            start_column: i + 1,
                            length: j - i,
                        });
                        i = j;
                    } else {
                        self.error(line + 1, &format!("Unexpected character: {}", char));
                        i += 1;
                    }
                }
            }
        }

        tokens.push(Token {
            inner: TokenType::EOF,
            lexeme: String::new(),
            line: 0,
            start_column: 0,
            length: 0,
        });
        self.tokens = tokens;
    }
}
