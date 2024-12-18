use instructions::Instruction;
pub use op_codes::OpCode;
pub use values::Value;

use crate::interpreter::{
    lexer,
    parser::{
        self,
        ast::{Expr, Stmt},
    },
    resolver, SourceCodeRange,
};

mod chunk;
pub(crate) mod disassembler;
mod instructions;
mod op_codes;
mod values;
pub use chunk::Chunk;

#[derive(Debug)]
pub enum CompilerError {
    LexError,
    ParseError(()),
    // ResolverError(resolver::ResolverError),
    CompileError(CompileError),
    ExecError(()),
    ResolverError(()),
}

#[derive(Debug)]
pub enum CompileError {
    LiteralToValueError,
}

impl From<CompileError> for CompilerError {
    fn from(err: CompileError) -> Self {
        CompilerError::CompileError(err)
    }
}

impl From<values::LiteralToValueError> for CompileError {
    fn from(_: values::LiteralToValueError) -> Self {
        CompileError::LiteralToValueError
    }
}

pub fn compile(input: &str) -> Result<Chunk, CompilerError> {
    let mut chunk = Chunk::new();
    // let idx = chunk.addConstant(Value::Number(1.2));
    // chunk.writeChunk(OpCode::OpConstant as u8, SouceCodeRange::new(0));
    // chunk.writeChunk(idx as u8, SouceCodeRange::new(0));
    // chunk.writeChunk(OpCode::OpReturn as u8, SouceCodeRange::new(1));
    // chunk.writeChunk(OpCode::OpReturn as u8, SouceCodeRange::new(1));

    let tokens = lexer::tokenize(input).map_err(|_| CompilerError::LexError)?;

    let mut parser = parser::ParserInstance::new(tokens);
    let stmnts = parser.parse().map_err(|_| CompilerError::ParseError(()))?;
    // let mut resolver = resolver::Resolver::new();
    // resolver
    //     .resolve(&stmnts)
    //     .map_err(|_| CompilerError::ResolverError(()))?;
    // let expr = parser
    //     .parse_expr()
    //     .map_err(|_| CompilerError::ParseError(()))?;
    // let mut resolver = resolver::Resolver::new();
    // resolver
    //     .resolve_expr(&expr)
    //     .map_err(|_| CompilerError::ParseError(()))?;

    // chunk.add_instruction(
    //     Instruction::Constant(Value::Number(1.2)),
    //     SouceCodeRange::new(0),
    // );
    // chunk.add_instruction(Instruction::Negate, SouceCodeRange::new(1));
    // chunk.add_instruction(
    //     Instruction::Constant(Value::Number(1.2)),
    //     SouceCodeRange::new(0),
    // );
    // chunk.add_instruction(Instruction::Add, SouceCodeRange::new(2));
    // chunk.add_instruction(Instruction::Return, SouceCodeRange::new(2));

    for stmt in &stmnts {
        stmt.compile(&mut chunk)?;
    }
    chunk.add_instruction(Instruction::Return, SourceCodeRange::new(2));
    Ok(chunk)
}

trait Compile {
    fn compile(&self, chunk: &mut Chunk) -> Result<(), CompileError>;
}

impl Compile for Stmt {
    fn compile(&self, chunk: &mut Chunk) -> Result<(), CompileError> {
        use crate::interpreter::parser::ast::StmtType::*;
        match &self.intern {
            Expr(expr) => {
                expr.compile(chunk)?;
                chunk.add_instruction(Instruction::Pop, self.range);
            }
            Print(expr) => {
                expr.compile(chunk)?;
                chunk.add_instruction(Instruction::Print, self.range);
            }
            Var(name, expr) => {
                if let Some(expr) = expr {
                    expr.compile(chunk)?;
                } else {
                    chunk.add_instruction(Instruction::Constant(Value::Nil), self.range);
                }
                chunk.add_instruction(Instruction::DefineGlobal(ustr::ustr(name)), self.range);
            }
            _ => todo!(),
        }
        Ok(())
    }
}

impl Compile for Expr {
    fn compile(&self, chunk: &mut Chunk) -> Result<(), CompileError> {
        use crate::interpreter::parser::ast::ExprType::*;
        match &*self.intern {
            Literal(value) => {
                chunk.add_instruction(
                    Instruction::Constant(value.clone().try_into()?),
                    SourceCodeRange::new(0),
                );
            }
            Grouping(expr) => {
                expr.compile(chunk)?;
            }
            Unary(unary) => {
                unary.compile(chunk)?;
                match unary.intern {
                    parser::ast::UnaryType::Not => {
                        chunk.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::UnaryType::Neg => {
                        chunk.add_instruction(Instruction::Negate, self.range);
                    }
                }
            }
            Binary(binary) => {
                binary.left.compile(chunk)?;
                binary.right.compile(chunk)?;
                match binary.operator {
                    parser::ast::Operator::EqualEqual => {
                        chunk.add_instruction(Instruction::Equal, self.range);
                    }
                    parser::ast::Operator::NEqualEqual => {
                        chunk.add_instruction(Instruction::Equal, self.range);
                        chunk.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::Operator::Less => {
                        chunk.add_instruction(Instruction::Less, self.range);
                    }
                    parser::ast::Operator::Leq => {
                        chunk.add_instruction(Instruction::Greater, self.range);
                        chunk.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::Operator::Greater => {
                        chunk.add_instruction(Instruction::Greater, self.range);
                    }
                    parser::ast::Operator::Greq => {
                        chunk.add_instruction(Instruction::Less, self.range);
                        chunk.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::Operator::Plus => {
                        chunk.add_instruction(Instruction::Add, self.range);
                    }
                    parser::ast::Operator::Minus => {
                        chunk.add_instruction(Instruction::Subtract, self.range);
                    }
                    parser::ast::Operator::Times => {
                        chunk.add_instruction(Instruction::Multiply, self.range);
                    }
                    parser::ast::Operator::Div => {
                        chunk.add_instruction(Instruction::Divide, self.range);
                    }
                }
            }
            Logical(logical) => todo!(),
            Variable(var) => {
                chunk.add_instruction(Instruction::GetGlobal(ustr::ustr(var)), self.range);
            }
            Assign(name, expr) => {
                expr.compile(chunk)?;
                chunk.add_instruction(Instruction::SetGlobal(ustr::ustr(name)), self.range);
            }
            Call(call) => todo!(),
            Get(expr, _) => todo!(),
            Set(expr, _, expr1) => todo!(),
        }
        Ok(())
    }
}

impl Compile for parser::ast::Unary {
    fn compile(&self, chunk: &mut Chunk) -> Result<(), CompileError> {
        self.expr.compile(chunk)?;
        Ok(())
    }
}
