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
                char => {
                    self.error(line + 1, &format!("Unexpected character: {}", char));
                    i += 1;
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
