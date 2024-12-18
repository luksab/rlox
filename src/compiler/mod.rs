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
    VariableAlreadyDefined,
    VariableNotDefined,
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

struct Local {
    name: String,
    depth: i32,
}

pub struct Compiler {
    locals: Vec<Local>,
    scope_depth: i32,
    chunk: Chunk,
    current_range: SourceCodeRange,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            scope_depth: 0,
            chunk: Chunk::new(),
            current_range: SourceCodeRange::new(0),
        }
    }

    pub fn compile(&mut self, input: &str) -> Result<(), CompilerError> {
        let tokens = lexer::tokenize(input).map_err(|_| CompilerError::LexError)?;

        let mut parser = parser::ParserInstance::new(tokens);
        let stmnts = parser.parse().map_err(|_| CompilerError::ParseError(()))?;

        for stmt in &stmnts {
            self.current_range = stmt.range;
            stmt.compile(self)?;
        }
        self.add_instruction(Instruction::Return, SourceCodeRange::new(2));
        Ok(())
    }

    pub fn into_chunk(self) -> Chunk {
        self.chunk
    }

    pub fn add_instruction(&mut self, instruction: Instruction, range: SourceCodeRange) {
        if let Ok(op) = OpCode::try_from(&instruction) {
            self.chunk.push_code(op as u8, range);
        } else {
            use Instruction::*;
            match instruction {
                Constant(value) => {
                    let idx = self.chunk.constant_pool.len();
                    self.chunk.constant_pool.push(value);

                    if idx > 255 {
                        self.chunk.push_code(OpCode::OpConstantLong as u8, range);
                        self.chunk.push_code((idx >> 16) as u8, range);
                        self.chunk.push_code((idx >> 8) as u8, range);
                        self.chunk.push_code(idx as u8, range);
                        return;
                    } else {
                        self.chunk.push_code(OpCode::OpConstant as u8, range);
                        self.chunk.push_code(idx as u8, range);
                    }
                }
                DefineGlobal(u) => {
                    self.chunk.push_code(OpCode::OpDefineGlobal as u8, range);
                    let pointer_address = u.as_ptr() as usize;
                    // push all the bytes of the pointer address
                    for i in 0..std::mem::size_of::<usize>() {
                        self.chunk
                            .push_code((pointer_address >> (i * 8)) as u8, range);
                    }
                }
                GetGlobal(u) => {
                    self.chunk.push_code(OpCode::OpGetGlobal as u8, range);
                    let pointer_address = u.as_ptr() as usize;
                    // push all the bytes of the pointer address
                    for i in 0..std::mem::size_of::<usize>() {
                        self.chunk
                            .push_code((pointer_address >> (i * 8)) as u8, range);
                    }
                }
                SetGlobal(u) => {
                    self.chunk.push_code(OpCode::OpSetGlobal as u8, range);
                    let pointer_address = u.as_ptr() as usize;
                    // push all the bytes of the pointer address
                    for i in 0..std::mem::size_of::<usize>() {
                        self.chunk
                            .push_code((pointer_address >> (i * 8)) as u8, range);
                    }
                }
                GetLocal(idx) => {
                    self.chunk.push_code(OpCode::OpGetLocal as u8, range);
                    self.chunk.push_code(idx, range);
                }
                SetLocal(idx) => {
                    self.chunk.push_code(OpCode::OpSetLocal as u8, range);
                    self.chunk.push_code(idx, range);
                }
                Jump(idx) => {
                    self.chunk.push_code(OpCode::OpJump as u8, range);
                    self.chunk.push_code((idx >> 8) as u8, range);
                    self.chunk.push_code(idx as u8, range);
                }
                JumpIfFalse(idx) => {
                    self.chunk.push_code(OpCode::OpJumpIfFalse as u8, range);
                    self.chunk.push_code((idx >> 8) as u8, range);
                    self.chunk.push_code(idx as u8, range);
                }
                instr => {
                    unreachable!(
                        "All instructions should either be try_from or handled in the match block. Got {:?}",
                        instr
                    )
                }
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.locals.pop();
                self.add_instruction(Instruction::Pop, self.current_range);
            } else {
                break;
            }
        }
    }

    fn add_local(&mut self, name: String) {
        self.locals.push(Local {
            name,
            depth: self.scope_depth,
        });
    }

    fn resolve_local(&mut self, name: &str) -> Result<usize, CompileError> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Ok(i);
            }
        }
        Err(CompileError::VariableNotDefined)
    }

    fn emit_jump(&mut self, instruction: Instruction) -> usize {
        assert!(matches!(
            instruction,
            Instruction::Jump(_) | Instruction::JumpIfFalse(_)
        ));
        self.add_instruction(instruction, self.current_range);
        self.chunk.code_array.len() - 2
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code_array.len() - offset - 2;
        if offset > 0xffff {
            panic!("Too much code to jump over");
        }
        self.chunk.code_array[offset] = (jump >> 8) as u8;
        self.chunk.code_array[offset + 1] = jump as u8;
    }
}

trait Compile {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompileError>;
}

impl Compile for Stmt {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        use crate::interpreter::parser::ast::StmtType::*;
        match &self.intern {
            Expr(expr) => {
                expr.compile(compiler)?;
                compiler.add_instruction(Instruction::Pop, self.range);
            }
            Print(expr) => {
                expr.compile(compiler)?;
                compiler.add_instruction(Instruction::Print, self.range);
            }
            Var(name, expr) => {
                if let Some(expr) = expr {
                    expr.compile(compiler)?;
                } else {
                    compiler.add_instruction(Instruction::Constant(Value::Nil), self.range);
                }
                if compiler.scope_depth > 0 {
                    // check that the variable is not already defined in the current scope
                    for local in compiler.locals.iter().rev() {
                        if local.depth != -1 && local.depth < compiler.scope_depth {
                            break;
                        }
                        if local.name == *name {
                            return Err(CompileError::VariableAlreadyDefined);
                        }
                    }
                    compiler.add_local(name.clone());
                } else {
                    compiler
                        .add_instruction(Instruction::DefineGlobal(ustr::ustr(name)), self.range);
                }
            }
            Block(stmts) => {
                compiler.begin_scope();
                for stmt in stmts {
                    stmt.compile(compiler)?;
                }
                compiler.end_scope();
            }
            IfStmt(cond, then_branch, else_branch) => {
                cond.compile(compiler)?;
                let jump = compiler.emit_jump(Instruction::JumpIfFalse(0));
                compiler.add_instruction(Instruction::Pop, self.range);
                then_branch.compile(compiler)?;
                let end = compiler.emit_jump(Instruction::Jump(0));
                compiler.patch_jump(jump);
                compiler.add_instruction(Instruction::Pop, self.range);

                if let Some(else_branch) = else_branch {
                    else_branch.compile(compiler)?;
                }
                compiler.patch_jump(end);
            }
            _ => todo!(),
        }
        Ok(())
    }
}

impl Compile for Expr {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        use crate::interpreter::parser::ast::ExprType::*;
        match &*self.intern {
            Literal(value) => {
                compiler.add_instruction(
                    Instruction::Constant(value.clone().try_into()?),
                    SourceCodeRange::new(0),
                );
            }
            Grouping(expr) => {
                expr.compile(compiler)?;
            }
            Unary(unary) => {
                unary.compile(compiler)?;
                match unary.intern {
                    parser::ast::UnaryType::Not => {
                        compiler.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::UnaryType::Neg => {
                        compiler.add_instruction(Instruction::Negate, self.range);
                    }
                }
            }
            Binary(binary) => {
                binary.left.compile(compiler)?;
                binary.right.compile(compiler)?;
                match binary.operator {
                    parser::ast::Operator::EqualEqual => {
                        compiler.add_instruction(Instruction::Equal, self.range);
                    }
                    parser::ast::Operator::NEqualEqual => {
                        compiler.add_instruction(Instruction::Equal, self.range);
                        compiler.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::Operator::Less => {
                        compiler.add_instruction(Instruction::Less, self.range);
                    }
                    parser::ast::Operator::Leq => {
                        compiler.add_instruction(Instruction::Greater, self.range);
                        compiler.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::Operator::Greater => {
                        compiler.add_instruction(Instruction::Greater, self.range);
                    }
                    parser::ast::Operator::Greq => {
                        compiler.add_instruction(Instruction::Less, self.range);
                        compiler.add_instruction(Instruction::Not, self.range);
                    }
                    parser::ast::Operator::Plus => {
                        compiler.add_instruction(Instruction::Add, self.range);
                    }
                    parser::ast::Operator::Minus => {
                        compiler.add_instruction(Instruction::Subtract, self.range);
                    }
                    parser::ast::Operator::Times => {
                        compiler.add_instruction(Instruction::Multiply, self.range);
                    }
                    parser::ast::Operator::Div => {
                        compiler.add_instruction(Instruction::Divide, self.range);
                    }
                }
            }
            Logical(logical) => {
                logical.left.compile(compiler)?;
                match logical.operator {
                    parser::ast::LogicalOperator::And => {
                        let jump = compiler.emit_jump(Instruction::JumpIfFalse(0));
                        compiler.add_instruction(Instruction::Pop, self.range);
                        logical.right.compile(compiler)?;
                        compiler.patch_jump(jump);
                    }
                    parser::ast::LogicalOperator::Or => {
                        let else_jump = compiler.emit_jump(Instruction::JumpIfFalse(0));
                        let end_jump = compiler.emit_jump(Instruction::Jump(0));
                        compiler.patch_jump(else_jump);
                        compiler.add_instruction(Instruction::Pop, self.range);
                        logical.right.compile(compiler)?;
                        compiler.patch_jump(end_jump);
                    }
                }
            }
            Variable(name) => {
                if let Ok(idx) = compiler.resolve_local(name) {
                    compiler.add_instruction(
                        Instruction::GetLocal(idx.try_into().unwrap()),
                        self.range,
                    );
                } else {
                    compiler.add_instruction(Instruction::GetGlobal(ustr::ustr(name)), self.range);
                }
            }
            Assign(name, expr) => {
                expr.compile(compiler)?;
                if let Ok(idx) = compiler.resolve_local(name) {
                    compiler.add_instruction(
                        Instruction::SetLocal(idx.try_into().unwrap()),
                        self.range,
                    );
                } else {
                    compiler.add_instruction(Instruction::SetGlobal(ustr::ustr(name)), self.range);
                }
            }
            Call(call) => todo!(),
            Get(expr, _) => todo!(),
            Set(expr, _, expr1) => todo!(),
        }
        Ok(())
    }
}

impl Compile for parser::ast::Unary {
    fn compile(&self, compiler: &mut Compiler) -> Result<(), CompileError> {
        self.expr.compile(compiler)?;
        Ok(())
    }
}
