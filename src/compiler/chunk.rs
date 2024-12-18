use std::collections::HashMap;

use super::{SourceCodeRange, Value};

pub struct Chunk {
    pub(crate) code_array: Vec<u8>,
    pub(crate) constant_pool: Vec<Value>,
    pub(crate) globals: HashMap<ustr::Ustr, Value>,
    pub(crate) lines: Vec<SourceCodeRange>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code_array: Vec::new(),
            constant_pool: Vec::new(),
            globals: HashMap::new(),
            lines: Vec::new(),
        }
    }

    pub fn push_code(&mut self, code: u8, line: SourceCodeRange) {
        self.code_array.push(code);
        self.lines.push(line);
    }
}
