use crate::SyntaxResult;
use ast::ListComparison;
use syntax::ListComparisonNode;

pub fn lower_list_comparison(node: ListComparisonNode) -> SyntaxResult<ListComparison> {
    let parameter = match node.parameter() {
        Some(parameter) => crate::lower_question_parameter(parameter)?,
        None => invalidate!("Expected a parameter"),
    };
    let list = match node.list() {
        Some(list) => crate::lower_list_literal(list)?,
        None => invalidate!("Expected a list"),
    };
    Ok(ListComparison::new(
        node.syntax(),
        node.parent(),
        parameter,
        list,
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
    fn empty() {
        assert_snapshot!("Q123 IN []");
    }

    #[test]
    fn numeric() {
        assert_snapshot!("Q123 IN [1]");
    }

    #[test]
    fn string() {
        assert_snapshot!("Q123 IN [\"foo\"]");
    }

    #[test]
    fn complex() {
        assert_snapshot!("Q123 IN [1, \"foo\"]");
    }
}
