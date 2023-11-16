use crate::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn singular_literal(&mut self) -> State {
        one_of!(self.numeric_literal(), self.string_literal(),)
    }

    pub fn list_literal(&mut self) -> State {
        if self.peek() != SyntaxKind::LEFT_BRACKET {
            return State::Continue;
        }

        self.start_node(SyntaxKind::LIST_LITERAL_NODE);
        self.bump();
        self.skip_whitespace();

        while !self.is_empty() {
            if self.singular_literal() == State::Continue {
                break;
            }

            self.skip_whitespace();
            if self.peek() == SyntaxKind::COMMA {
                self.bump();
                self.skip_whitespace();
            }
        }

        if self.peek() == SyntaxKind::RIGHT_BRACKET {
            self.bump();
        }

        self.finish_node();
        State::Stop
    }

    pub fn numeric_literal(&mut self) -> State {
        match self.peek() {
            SyntaxKind::NUMERIC => {
                self.start_node(SyntaxKind::NUMERIC_LITERAL_NODE);
                self.bump();
                self.finish_node();
                State::Stop
            }
            _ => State::Continue,
        }
    }

    pub fn string_literal(&mut self) -> State {
        match self.peek() {
            SyntaxKind::STRING => {
                self.start_node(SyntaxKind::STRING_LITERAL_NODE);
                self.bump();
                self.finish_node();
                State::Stop
            }
            _ => State::Continue,
        }
    }

    pub fn date_literal(&mut self) -> State {
        match self.peek() {
            SyntaxKind::STRING => {
                self.start_node(SyntaxKind::DATE_LITERAL_NODE);
                self.bump();
                self.finish_node();
                State::Stop
            }
            _ => State::Continue,
        }
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
    fn singular() {
        assert_snapshot!(singular_literal(r#"123"#));
        assert_snapshot!(singular_literal(r#""abc""#));
    }

    #[test]
    fn string() {
        assert_snapshot!(string_literal(r#""abc""#));
    }

    #[test]
    fn numeric() {
        assert_snapshot!(numeric_literal(r#"123"#));
    }

    #[test]
    fn list() {
        assert_snapshot!(list_literal(r#"[123, "abc"]"#));
    }

    #[test]
    fn date() {
        assert_snapshot!(date_literal(r#""2020-01-01""#));
    }
}
