mod error;
mod option;

use ast_semantic::SemanticOption;
pub use error::*;
pub use option::*;

pub fn compile(option: CompileOption) -> Result<(), CompileError> {
    for question in option.questions.values() {
        let tokens = lex::lex(&question.logic);
        let node = parse::parse(tokens);
        let ast = syntax_lower::lower(node)?;
        ast_semantic::check(&SemanticOption {
            questions: &option.questions,
            node: &ast,
        })?;
    }
    Ok(())
}

#[test]
fn semantic() {
    use entity::{Question, QuestionId};
    use std::collections::HashMap;

    macro_rules! questions {
        ($($id:expr => $logic:expr,)*) => {{
            let mut map = HashMap::new();
            $(
                let id = QuestionId($id);
                let logic = $logic.to_owned();
                map.insert(id, Question::new(id, logic));
            )*
            map
        }};
    }

    let questions = questions! {
        123 => "Q123 = 1",
        456 => "Q123 = 1 Q456 = 2",
        789 => "Q123 = 1 Q456 = 2 Q789 = 3",
    };
    let result = compile(CompileOption { questions });
    assert!(result.is_ok());
}
