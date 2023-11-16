use crate::SyntaxResult;
use ast::{BinaryComparison, BinaryComparisonOperator, BinaryComparisonOperatorKind};
use syntax::{BinaryComparisonNode, BinaryComparisonOperatorNode};
use syntax_kind::SyntaxKind;

pub fn lower_binary_comparison(node: BinaryComparisonNode) -> SyntaxResult<BinaryComparison> {
    let parameter = match node.parameter() {
        Some(parameter) => crate::lower_question_parameter(parameter)?,
        None => invalidate!("Expected a parameter"),
    };
    let operator = match node.operator() {
        Some(operator) => crate::lower_binary_comparison_operator(operator)?,
        None => invalidate!("Expected a comparison operator"),
    };
    let value = match node.value() {
        Some(value) => crate::lower_singular_literal(value)?,
        None => invalidate!("Expected a value"),
    };
    Ok(BinaryComparison::new(
        node.syntax(),
        node.parent(),
        parameter,
        operator,
        value,
    ))
}

pub fn lower_binary_comparison_operator(
    node: BinaryComparisonOperatorNode,
) -> SyntaxResult<BinaryComparisonOperator> {
    let kind = match node.syntax().kind() {
        SyntaxKind::EQUAL_OPERATOR_NODE => BinaryComparisonOperatorKind::Equal,
        SyntaxKind::NOT_EQUAL_OPERATOR_NODE => BinaryComparisonOperatorKind::NotEqual,
        SyntaxKind::LESS_THAN_OPERATOR_NODE => BinaryComparisonOperatorKind::LessThan,
        SyntaxKind::GREATER_THAN_OPERATOR_NODE => BinaryComparisonOperatorKind::GreaterThan,
        SyntaxKind::LESS_THAN_OR_EQUAL_OPERATOR_NODE => {
            BinaryComparisonOperatorKind::LessThanOrEqual
        }
        SyntaxKind::GREATER_THAN_OR_EQUAL_OPERATOR_NODE => {
            BinaryComparisonOperatorKind::GreaterThanOrEqual
        }
        _ => invalidate!("Expected a comparison operator"),
    };
    Ok(BinaryComparisonOperator::new(
        node.syntax(),
        node.parent(),
        kind,
    ))
}

#[cfg(test)]
mod tests {
    macro_rules! assert_snapshot {
        ($source:expr) => {
            let tokens = lex::lex($source);
            let node = parse::parse(tokens);
            let ast = crate::lower(node);
            insta::assert_debug_snapshot!(ast);
        };
    }

    #[test]
    fn numeric() {
        assert_snapshot!("Q123 = 1");
    }

    #[test]
    fn string() {
        assert_snapshot!("Q123 = \"foo\"");
    }
}
