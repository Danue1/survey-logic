use crate::SyntaxResult;
use ast::Statement;
use syntax::StatementNode;

pub fn lower_statement(node: StatementNode) -> SyntaxResult<Statement> {
    let condition = match node.condition() {
        Some(condition) => crate::lower_condition(condition)?,
        None => invalidate!("Expected a condition"),
    };
    Ok(Statement::new(node.syntax(), node.parent(), condition))
}
