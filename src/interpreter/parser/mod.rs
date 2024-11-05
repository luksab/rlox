use std::{backtrace::Backtrace, fmt::Display};

use super::token::{Token, TokenType};

pub(crate) mod ast;
use ast::*;

#[derive(Debug)]
pub struct ParserError {
    pub(crate) message: String,
    pub(crate) token: Token,
    #[allow(dead_code)]
    pub(crate) backtrace: Backtrace,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "{}", self.backtrace)?;
        write!(f, "{} at line {}", self.message, self.token.range.line)
    }
}

type Result<T> = std::result::Result<T, ParserError>;

pub struct ParserInstance {
    pub current: usize,
    pub tokens: Vec<Token>,
}

impl ParserInstance {
    #[allow(dead_code)]
    pub fn print_remaining(&self) {
        println!(
            "Tokens left: {:?}",
            self.tokens[self.current..].iter().collect::<Vec<_>>()
        );
    }

    fn error(&mut self, token: &Token, message: &str) {
        match token.inner {
            TokenType::EOF => self.report(token.range.line, " at end", message),
            _ => self.report(token.range.line, "", message),
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        // match self.expression() {
        //     Ok(expr) => Ok(expr),
        //     Err(err) => {
        //         self.error(&err.token, &err.message);
        //         self.synchronize();
        //         return Err(err);
        //     }
        // }
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    self.error(&err.token, &err.message);
                    self.synchronize();
                }
            }
        }

        return Ok(statements);
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
            backtrace: Backtrace::force_capture(),
        })
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.mtch(vec![TokenType::Print]) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        return Ok(Stmt {
            range: value.range.clone(),
            intern: StmtType::Print(value),
        });
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        return Ok(Stmt {
            range: expr.range.clone(),
            intern: StmtType::Expr(expr),
        });
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
                range: expr.range.merge(&right.range).merge(&self.previous().range),
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
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(err) => {
                self.advance();
                // consume the token on the right side of the binary operator
                while self.mtch(vec![
                    TokenType::Greater,
                    TokenType::GreaterEqual,
                    TokenType::Less,
                    TokenType::LessEqual,
                ]) {
                    let right = self.term()?;
                    println!("Discarding Right: {}", right);
                }

                // self.print_remaining();

                if err.token.inner.is_binary() {
                    return Err(ParserError {
                        message: "binary operator appearing at the beginning of an expression."
                            .to_string(),
                        token: err.token,
                        backtrace: Backtrace::force_capture(),
                    });
                }
                return Err(err);
            }
        };

        while self.mtch(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().inner.clone();
            let right = self.term()?;
            expr = Expr {
                range: expr.range.merge(&right.range).merge(&self.previous().range),
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
                range: expr.range.merge(&right.range).merge(&self.previous().range),
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
                range: expr.range.merge(&right.range).merge(&self.previous().range),
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
                range: right.range.merge(&self.previous().range),
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
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Literal(Literal::False)),
            });
        }
        if self.mtch(vec![TokenType::True]) {
            return Ok(Expr {
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Literal(Literal::True)),
            });
        }
        if self.mtch(vec![TokenType::Nil]) {
            return Ok(Expr {
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Literal(Literal::Nil)),
            });
        }

        match self.peek().inner.clone() {
            TokenType::Number(n) => {
                self.advance();
                return Ok(Expr {
                    range: self.previous().range.clone(),
                    intern: Box::new(ExprType::Literal(Literal::Number(n))),
                });
            }
            TokenType::String(s) => {
                self.advance();
                return Ok(Expr {
                    range: self.previous().range.clone(),
                    intern: Box::new(ExprType::Literal(Literal::String(s))),
                });
            }
            _ => (),
        }

        if self.mtch(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr {
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Grouping(expr)),
            });
        }

        Err(ParserError {
            message: "Expect expression.".to_string(),
            token: self.peek().to_owned(),
            backtrace: Backtrace::force_capture(),
        })
    }

    fn mtch(&mut self, types: Vec<TokenType>) -> bool {
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
