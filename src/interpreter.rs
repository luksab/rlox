mod token;

use token::{Token, TokenType};

pub struct InterpreterInstance {
    pub had_error: bool,
    pub tokens: Vec<Token>,
}

impl InterpreterInstance {
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = true;
    }

    pub fn new() -> Self {
        InterpreterInstance {
            had_error: false,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self, source: &str) {
        let mut tokens = Vec::new();
        for (line_num, line) in source.lines().enumerate() {
            let chars: Vec<_> = line.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let char = chars[i];
                match char {
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
                            line: line_num + 1,
                            start_column: i + 1,
                            length: 1,
                        });
                        i += 1;
                    }
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
                                    self.error(line_num + 1, "Unexpected character");
                                    (0, TokenType::EOF)
                                }
                            };
                            tokens.push(Token {
                                inner: token_type,
                                lexeme: chars[i..i + len].iter().collect(),
                                line: line_num + 1,
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
                                line: line_num + 1,
                                start_column: i + 1,
                                length: 1,
                            });
                            i += 1;
                        }
                    }
                    char => {
                        self.error(line_num + 1, &format!("Unexpected character: {}", char));
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
