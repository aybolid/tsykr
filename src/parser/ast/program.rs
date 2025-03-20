use super::{Node, Statement};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }

    pub fn push_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        "".to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
