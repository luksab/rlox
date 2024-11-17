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

struct ExprIdCounter {
    counter: usize,
}

impl ExprIdCounter {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn next(&mut self) -> ExprId {
        self.counter += 1;
        return ExprId(self.counter);
    }
}

pub struct ParserInstance {
    pub current: usize,
    pub had_error: bool,
    pub tokens: Vec<Token>,
    exp_id_counter: ExprIdCounter,
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
        self.had_error = true;
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens,
            had_error: false,
            exp_id_counter: ExprIdCounter::new(),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(err) => {
                self.error(&err.token, &err.message);
                self.synchronize();
                return Err(err);
            }
        }
    }

    pub fn parse(&mut self) -> std::result::Result<Vec<Stmt>, ()> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let declaration = self.declaration();
            match declaration {
                Ok(declaration) => statements.push(declaration),
                Err(err) => {
                    self.error(&err.token, &err.message);
                    self.synchronize();
                }
            }
        }

        if self.had_error {
            return Err(());
        } else {
            return Ok(statements);
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
            backtrace: Backtrace::force_capture(),
        })
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.mtch(vec![TokenType::Fun]) {
            return self.function(FunctionType::Function);
        }
        if self.mtch(vec![TokenType::Var]) {
            return self.var_declaration();
        }

        return self.statement();
    }

    fn function(&mut self, kind: FunctionType) -> Result<Stmt> {
        let name = match self.peek().inner {
            TokenType::Identifier(_) => self.advance(),
            _ => {
                return Err(ParserError {
                    message: format!("Expect {} name.", kind.tipe()),
                    token: self.peek().to_owned(),
                    backtrace: Backtrace::force_capture(),
                })
            }
        };
        let range = name.range.clone();
        let name = if let TokenType::Identifier(name) = &name.inner {
            name.clone()
        } else {
            unreachable!()
        };

        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind.tipe()).as_str(),
        )?;
        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    self.error(
                        &self.peek().clone(),
                        "Cannot have more than 255 parameters.",
                    );
                }

                let param = match self.peek().inner {
                    TokenType::Identifier(_) => self.advance(),
                    _ => {
                        return Err(ParserError {
                            message: "Expect parameter name.".to_string(),
                            token: self.peek().to_owned(),
                            backtrace: Backtrace::force_capture(),
                        })
                    }
                };
                let param = if let TokenType::Identifier(param) = &param.inner {
                    param.clone()
                } else {
                    unreachable!()
                };
                parameters.push(param);

                if !self.mtch(vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body.", kind.tipe()).as_str(),
        )?;
        let body = self.block_statement()?;

        return Ok(Stmt {
            range: range.merge(&body.range),
            intern: StmtType::Function(kind, name, parameters, Box::new(body)),
        });
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = match self.peek().inner {
            TokenType::Identifier(_) => self.advance(),
            _ => {
                return Err(ParserError {
                    message: "Expect variable name.".to_string(),
                    token: self.peek().to_owned(),
                    backtrace: Backtrace::force_capture(),
                })
            }
        };
        let range = name.range.clone();
        let name = if let TokenType::Identifier(name) = &name.inner {
            name.clone()
        } else {
            unreachable!()
        };

        let initializer = if self.mtch(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        return Ok(Stmt {
            range,
            intern: StmtType::Var(name, initializer),
        });
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.mtch(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.mtch(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.mtch(vec![TokenType::Return]) {
            return self.return_statement();
        }
        if self.mtch(vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.mtch(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.mtch(vec![TokenType::LeftBrace]) {
            return self.block_statement();
        }
        if self.mtch(vec![TokenType::Break]) {
            return self.break_statement();
        }
        if self.mtch(vec![TokenType::Continue]) {
            return self.continue_statement();
        }

        return self.expression_statement();
    }

    fn break_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
        return Ok(Stmt {
            range: self.previous().range.clone(),
            intern: StmtType::Break,
        });
    }

    fn continue_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::Semicolon, "Expect ';' after 'continue'.")?;
        return Ok(Stmt {
            range: self.previous().range.clone(),
            intern: StmtType::Continue,
        });
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);

        return Ok(Stmt {
            range: condition.range.merge(&body.range),
            intern: StmtType::While(condition, body),
        });
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.mtch(vec![TokenType::Semicolon]) {
            None
        } else if self.mtch(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr {
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Literal(Literal::True)),
                id: self.exp_id_counter.next(),
            }
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt {
                range: body.range.merge(&increment.range),
                intern: StmtType::Block(vec![
                    body,
                    Stmt {
                        range: increment.range.clone(),
                        intern: StmtType::Expr(increment),
                    },
                ]),
            };
        }

        body = Stmt {
            range: condition.range.merge(&body.range),
            intern: StmtType::While(condition, Box::new(body)),
        };

        if let Some(initializer) = initializer {
            body = Stmt {
                range: initializer.range.merge(&body.range),
                intern: StmtType::Block(vec![initializer, body]),
            };
        }

        return Ok(body);
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.mtch(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        return Ok(Stmt {
            range: condition
                .range
                .merge(&then_branch.range)
                .merge(&self.previous().range),
            intern: StmtType::IfStmt(condition, then_branch, else_branch),
        });
    }

    fn block_statement(&mut self) -> Result<Stmt> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        return Ok(Stmt {
            range: self.previous().range.clone(),
            intern: StmtType::Block(statements),
        });
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let keyword = self.previous().to_owned();
        let value = if self.check(TokenType::Semicolon) {
            Expr {
                range: keyword.range.clone(),
                intern: Box::new(ExprType::Literal(Literal::Nil)),
                id: self.exp_id_counter.next(),
            }
        } else {
            self.expression()?
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        return Ok(Stmt {
            range: keyword.range.merge(&value.range),
            intern: StmtType::Return(value),
        });
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
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;

        if self.mtch(vec![TokenType::Equal]) {
            let equals = self.previous().to_owned();
            let value = self.assignment()?;

            if let ExprType::Variable(ref name) = *expr.intern {
                return Ok(Expr {
                    range: equals.range.merge(&value.range),
                    intern: Box::new(ExprType::Assign(name.clone(), value)),
                    id: self.exp_id_counter.next(),
                });
            }

            // return Err(ParserError {
            //     message: "Invalid assignment target.".to_string(),
            //     token: equals,
            //     backtrace: Backtrace::force_capture(),
            // });
            self.error(&equals, "Invalid assignment target.");
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.mtch(vec![TokenType::Or]) {
            let operator = self.previous().inner.clone();
            let right = self.and()?;
            expr = Expr {
                range: expr.range.merge(&right.range).merge(&self.previous().range),
                intern: Box::new(ExprType::Logical(Logical {
                    left: expr,
                    operator: (&operator).into(),
                    right: right,
                })),
                id: self.exp_id_counter.next(),
            };
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.mtch(vec![TokenType::And]) {
            let operator = self.previous().inner.clone();
            let right = self.equality()?;
            expr = Expr {
                range: expr.range.merge(&right.range).merge(&self.previous().range),
                intern: Box::new(ExprType::Logical(Logical {
                    left: expr,
                    operator: (&operator).into(),
                    right: right,
                })),
                id: self.exp_id_counter.next(),
            };
        }

        return Ok(expr);
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
                id: self.exp_id_counter.next(),
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
                id: self.exp_id_counter.next(),
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
                id: self.exp_id_counter.next(),
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
                id: self.exp_id_counter.next(),
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
                id: self.exp_id_counter.next(),
            });
        }

        return self.call();
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.mtch(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            arguments.push(self.expression()?);
            while self.mtch(vec![TokenType::Comma]) {
                arguments.push(self.expression()?);
                if arguments.len() >= 255 {
                    self.error(&self.peek().clone(), "Cannot have more than 255 arguments.");
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        return Ok(Expr {
            range: callee.range.merge(&paren.range),
            intern: Box::new(ExprType::Call(Call {
                callee: callee,
                arguments,
            })),
            id: self.exp_id_counter.next(),
        });
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //            | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr> {
        if self.mtch(vec![TokenType::False]) {
            return Ok(Expr {
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Literal(Literal::False)),
                id: self.exp_id_counter.next(),
            });
        }
        if self.mtch(vec![TokenType::True]) {
            return Ok(Expr {
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Literal(Literal::True)),
                id: self.exp_id_counter.next(),
            });
        }
        if self.mtch(vec![TokenType::Nil]) {
            return Ok(Expr {
                range: self.previous().range.clone(),
                intern: Box::new(ExprType::Literal(Literal::Nil)),
                id: self.exp_id_counter.next(),
            });
        }

        match self.peek().inner.clone() {
            TokenType::Number(n) => {
                self.advance();
                return Ok(Expr {
                    range: self.previous().range.clone(),
                    intern: Box::new(ExprType::Literal(Literal::Number(n))),
                    id: self.exp_id_counter.next(),
                });
            }
            TokenType::String(s) => {
                self.advance();
                return Ok(Expr {
                    range: self.previous().range.clone(),
                    intern: Box::new(ExprType::Literal(Literal::String(s))),
                    id: self.exp_id_counter.next(),
                });
            }
            TokenType::Identifier(s) => {
                self.advance();
                return Ok(Expr {
                    range: self.previous().range.clone(),
                    intern: Box::new(ExprType::Variable(s)),
                    id: self.exp_id_counter.next(),
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
                id: self.exp_id_counter.next(),
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
