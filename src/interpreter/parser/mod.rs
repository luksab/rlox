use super::token::{Token, TokenType};

pub(crate) mod ast;
use ast::*;

#[derive(Debug)]
pub struct ParserError {
    pub(crate) message: String,
    pub(crate) token: Token,
}

type Result<T> = std::result::Result<T, ParserError>;

pub struct ParserInstance {
    pub current: usize,
    pub tokens: Vec<Token>,
}

impl ParserInstance {
    fn error(&mut self, token: &Token, message: &str) {
        match token.inner {
            TokenType::EOF => self.report(token.line, " at end", message),
            _ => self.report(token.line, "", message),
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().inner == TokenType::Semicolon {
                return;
            }

            match self.peek().inner {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(err) => {
                self.error(&err.token, &err.message);
                self.synchronize();
                return Err(err);
            }
        }
    }

    fn consume(&mut self, tipe: TokenType, message: &str) -> Result<&Token> {
        if self.check(tipe) {
            return Ok(self.advance());
        };

        let token = self.peek().to_owned();
        self.error(&token, message);
        Err(ParserError {
            message: "Parse error".to_string(),
            token,
        })
    }

    fn expression(&mut self) -> Result<Expr> {
        return self.equality();
    }

    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.mtch(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().inner.clone();
            let right = self.comparison()?;
            expr = Expr {
                intern: Box::new(ExprType::Binary(Binary {
                    left: expr,
                    operator: (&operator).into(),
                    right: right,
                })),
            }
        }

        return Ok(expr);
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.mtch(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().inner.clone();
            let right = self.term()?;
            expr = Expr {
                intern: Box::new(ExprType::Binary(Binary {
                    left: expr,
                    operator: (&operator).into(),
                    right: right,
                })),
            };
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        // while (match(MINUS, PLUS)) {
        while self.mtch(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().inner.clone();
            let right = self.factor()?;
            expr = Expr {
                intern: Box::new(ExprType::Binary(Binary {
                    left: expr,
                    operator: (&operator).into(),
                    right: right,
                })),
            };
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.mtch(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().inner.clone();
            let right = self.unary()?;
            expr = Expr {
                intern: Box::new(ExprType::Binary(Binary {
                    left: expr,
                    operator: (&operator).into(),
                    right: right,
                })),
            };
        }

        return Ok(expr);
    }

    // unary          → ( "!" | "-" ) unary
    //            | primary ;

    fn unary(&mut self) -> Result<Expr> {
        if self.mtch(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().inner.clone();
            let right = self.unary()?;
            return Ok(Expr {
                intern: Box::new(ExprType::Unary(Unary {
                    intern: (&operator).into(),
                    expr: right,
                })),
            });
        }

        return self.primary();
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //            | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr> {
        if self.mtch(vec![TokenType::False]) {
            return Ok(Expr {
                intern: Box::new(ExprType::Literal(Literal::False)),
            });
        }
        if self.mtch(vec![TokenType::True]) {
            return Ok(Expr {
                intern: Box::new(ExprType::Literal(Literal::True)),
            });
        }
        if self.mtch(vec![TokenType::Nil]) {
            return Ok(Expr {
                intern: Box::new(ExprType::Literal(Literal::Nil)),
            });
        }

        match self.peek().inner.clone() {
            TokenType::Number(n) => {
                self.advance();
                return Ok(Expr {
                    intern: Box::new(ExprType::Literal(Literal::Number(n))),
                });
            }
            TokenType::String(s) => {
                self.advance();
                return Ok(Expr {
                    intern: Box::new(ExprType::Literal(Literal::String(s))),
                });
            }
            _ => (),
        }

        if self.mtch(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr {
                intern: Box::new(ExprType::Grouping(expr)),
            });
        }

        Err(ParserError {
            message: "Expect expression.".to_string(),
            token: self.peek().to_owned(),
        })
    }

    fn mtch(&mut self, types: Vec<TokenType>) -> bool {
        // for (TokenType type : types) {
        //   if (check(type)) {
        //     advance();
        //     return true;
        //   }
        // }
        for tipe in types {
            if self.check(tipe) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, tipe: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        return self.peek().inner == tipe;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return matches!(self.peek().inner, TokenType::EOF);
    }

    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).unwrap();
    }

    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap();
    }
}
