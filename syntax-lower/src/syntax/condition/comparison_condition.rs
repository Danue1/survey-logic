use crate::SyntaxResult;
use ast::ComparisonCondition;
use syntax::ComparisonConditionNode;

pub fn lower_comparison_condition(
    node: ComparisonConditionNode,
) -> SyntaxResult<ComparisonCondition> {
    let comparison = match node.comparison() {
        Some(comparison) => crate::lower_comparison(comparison)?,
        None => invalidate!("Expected a comparison"),
    };
    Ok(ComparisonCondition::new(
        node.syntax(),
        node.parent(),
        comparison,
    ))
}
