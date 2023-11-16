use crate::context::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn question_parameter(&mut self) -> State {
        if self.peek_str().to_lowercase().as_str() != "q" {
            return State::Continue;
        }

        self.start_node(SyntaxKind::QUESTION_PARAMETER_NODE);
        self.bump();
        self.numeric_literal();
        self.finish_node();
        State::Stop
    }
}

#[cfg(test)]
mod tests {
    macro_rules! assert_snapshot {
        ($parse:ident($source:expr)) => {
            let tokens = lex::lex($source);
            let mut context = super::Context::new(tokens);
            context.$parse();
            let node = context.finish();
            insta::assert_debug_snapshot!(node);
        };
    }

    #[test]
    fn question_parameter() {
        assert_snapshot!(question_parameter("Q123"));
    }
}
