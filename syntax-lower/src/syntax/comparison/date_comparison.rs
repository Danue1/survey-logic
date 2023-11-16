use crate::SyntaxResult;
use ast::DateComparison;
use syntax::DateComparisonNode;

pub fn lower_date_comparison(node: DateComparisonNode) -> SyntaxResult<DateComparison> {
    let parameter = match node.parameter() {
        Some(parameter) => crate::lower_question_parameter(parameter)?,
        None => invalidate!("Expected a parameter"),
    };
    let value = match node.value() {
        Some(value) => crate::lower_date_literal(value)?,
        None => invalidate!("Expected a value"),
    };
    Ok(DateComparison::new(
        node.syntax(),
        node.parent(),
        parameter,
        value,
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
    fn before() {
        assert_snapshot!("Q123 BEFORE \"2023-11-15\"");
    }

    #[test]
    fn after() {
        assert_snapshot!("Q123 AFTER \"2023-11-15\"");
    }
}
