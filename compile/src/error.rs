use ast_semantic::SemanticError;
use syntax_lower::SyntaxError;

#[derive(Debug)]
pub enum CompileError {
    Syntax(SyntaxError),
    Semantic(SemanticError),
}

impl From<SyntaxError> for CompileError {
    fn from(error: SyntaxError) -> Self {
        Self::Syntax(error)
    }
}

impl From<SemanticError> for CompileError {
    fn from(error: SemanticError) -> Self {
        Self::Semantic(error)
    }
}
