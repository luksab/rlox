use std::collections::HashMap;

use super::{LoxCallable, LoxClass, LoxFunction};

use super::ExecResult;

#[derive(Clone, Default)]
pub(crate) struct LoxInstance {
    pub(crate) name: String,
    pub(crate) methods: HashMap<String, LoxFunction>,
    pub(crate) fields: HashMap<String, super::Literal>,
}

impl PartialEq for LoxInstance {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::fmt::Debug for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoxInstance")
            .field("name", &self.name)
            .field("methods", &self.methods)
            .finish()
    }
}

impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.name)
    }
}

impl LoxCallable for LoxInstance {
    fn call(
        &self,
        _args: Vec<super::Literal>,
        _ctx: &mut super::EvalCtx,
    ) -> Result<super::Literal, super::ExecError> {
        Ok(super::Literal::Nil)
    }

    fn arity_matches(&self, arity: usize) -> bool {
        arity == 0
    }

    fn print_arity(&self) -> String {
        "0".to_string()
    }
}

impl LoxInstance {
    pub(crate) fn new(class: &LoxClass) -> Self {
        let methods = HashMap::new();
        let fields = HashMap::new();
        // for stmt in methods {
        //     if let super::StmtType::Function(_, name, _, body) = stmt.intern {
        //         let function = LoxFunction::new(super::FunctionType::Method, name.clone(), body.clone());
        //         methods.insert(name, function);
        //     }
        // }
        Self {
            name: class.name.clone(),
            methods,
            fields,
        }
    }

    pub(crate) fn get(&self, name: &str) -> ExecResult<super::Literal> {
        self.fields
            .get(name)
            .ok_or(super::ExecError::new(
                format!("Undefined property '{}'.", name),
                super::SourceCodeRange {
                    line: 0,
                    start_column: 0,
                    length: 0,
                },
            ))
            .cloned()
    }

    pub(crate) fn set(
        &mut self,
        name: &str,
        value: super::Literal,
    ) -> Result<super::Literal, super::ExecError> {
        self.fields.insert(name.to_string(), value.clone());
        Ok(value)
    }
}
