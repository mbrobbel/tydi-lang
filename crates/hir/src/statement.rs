pub enum Statement {
    TypeDefinition { name: String },
}

impl Statement {
    pub fn lower(ast: tydi_lang_ast::Statement) -> Option<Self> {
        match ast {
            tydi_lang_ast::Statement::TypeDefinition(ast) => Some(Self::TypeDefinition {
                name: ast.name()?.text().to_string(),
            }),
        }
    }
}

pub enum Expr {}

pub enum Type {}
