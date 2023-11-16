use crate::SyntaxResult;
use ast::QuestionParameter;
use syntax::QuestionParameterNode;

pub fn lower_question_parameter(node: QuestionParameterNode) -> SyntaxResult<QuestionParameter> {
    let id = match node.id() {
        Some(id) => crate::lower_numeric_literal(id)?,
        None => invalidate!("Expected an identifier"),
    };
    Ok(QuestionParameter::new(node.syntax(), node.parent(), id))
}
