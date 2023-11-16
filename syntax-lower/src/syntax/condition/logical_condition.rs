use crate::SyntaxResult;
use ast::{LogicalCondition, LogicalConditionOperator, LogicalConditionOperatorKind};
use syntax::{LogicalConditionNode, LogicalConditionOperatorNode};
use syntax_kind::SyntaxKind;

pub fn lower_logical_condition(node: LogicalConditionNode) -> SyntaxResult<LogicalCondition> {
    let left = match node.left() {
        Some(left) => crate::lower_condition(left)?,
        None => invalidate!("Expected a left-hand side"),
    };
    let operator = match node.operator() {
        Some(operator) => lower_logical_condition_operator(operator)?,
        None => invalidate!("Expected an operator"),
    };
    let right = match node.right() {
        Some(right) => crate::lower_condition(right)?,
        None => invalidate!("Expected a right-hand side"),
    };
    Ok(LogicalCondition::new(
        node.syntax(),
        node.parent(),
        left,
        operator,
        right,
    ))
}

pub fn lower_logical_condition_operator(
    node: LogicalConditionOperatorNode,
) -> SyntaxResult<LogicalConditionOperator> {
    let kind = match node.syntax().kind() {
        SyntaxKind::AND_OPERATOR_NODE => LogicalConditionOperatorKind::And,
        SyntaxKind::OR_OPERATOR_NODE => LogicalConditionOperatorKind::Or,
        _ => invalidate!("Expected a logical operator"),
    };
    Ok(LogicalConditionOperator::new(
        node.syntax(),
        node.parent(),
        kind,
    ))
}
