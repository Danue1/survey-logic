mod condition;
mod literal;
mod question_parameter;
mod statement;

use crate::context::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn program(&mut self) {
        self.start_node(SyntaxKind::PROGRAM_NODE);
        self.skip_whitespace();
        while !self.is_empty() {
            if self.statement() == State::Continue {
                self.unexpected_token();
            }
            self.skip_whitespace();
        }
        self.finish_node();
    }

    pub fn unexpected_token(&mut self) {
        self.start_node(SyntaxKind::ERROR);
        self.bump();
        self.finish_node();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn statements() {
        let tokens = lex::lex("Q123 = 1 Q456 = 2");
        let mut context = super::Context::new(tokens);
        context.program();
        let node = context.finish();
        insta::assert_debug_snapshot!(node);
    }
}
