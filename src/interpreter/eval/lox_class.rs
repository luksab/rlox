use std::collections::HashMap;

use super::LoxFunction;

#[derive(Clone, Default)]
pub(crate) struct LoxClass {
    pub(crate) name: String,
    pub(crate) methods: HashMap<String, LoxFunction>,
}

impl PartialEq for LoxClass {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::fmt::Debug for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoxClass")
            .field("name", &self.name)
            .field("methods", &self.methods)
            .finish()
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl LoxClass {
    pub(crate) fn new(name: String, methods: Vec<super::Stmt>) -> Self {
        let mut methods = HashMap::new();
        // for stmt in methods {
        //     if let super::StmtType::Function(_, name, _, body) = stmt.intern {
        //         let function = LoxFunction::new(super::FunctionType::Method, name.clone(), body.clone());
        //         methods.insert(name, function);
        //     }
        // }
        Self { name, methods }
    }
}
