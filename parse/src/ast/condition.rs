use crate::context::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn logical_condition(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if self.atomic_condition() == State::Continue {
            return State::Continue;
        }

        while !self.is_empty() {
            self.skip_whitespace();
            if self.logical_operator() == State::Continue {
                break;
            }

            self.skip_whitespace();
            self.start_node_at(checkpoint, SyntaxKind::LOGICAL_CONDITION_NODE);
            self.atomic_condition();
            self.finish_node();
        }

        State::Stop
    }

    pub fn atomic_condition(&mut self) -> State {
        one_of!(self.group_condition(), self.binary_condition(),)
    }

    pub fn group_condition(&mut self) -> State {
        if self.peek() != SyntaxKind::LEFT_PAREN {
            return State::Continue;
        }

        self.start_node(SyntaxKind::GROUP_CONDITION_NODE);
        self.bump();
        self.skip_whitespace();
        if self.logical_condition() != State::Continue {
            self.skip_whitespace();
        }
        if self.peek() == SyntaxKind::RIGHT_PAREN {
            self.bump();
        }

        self.finish_node();
        State::Stop
    }

    pub fn logical_operator(&mut self) -> State {
        match self.peek_str().to_lowercase().as_str() {
            "and" => {
                self.start_node(SyntaxKind::AND_OPERATOR_NODE);
                self.bump();
                self.finish_node();
                State::Stop
            }
            "or" => {
                self.start_node(SyntaxKind::OR_OPERATOR_NODE);
                self.bump();
                self.finish_node();
                State::Stop
            }
            _ => State::Continue,
        }
    }

    pub fn binary_condition(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if self.question_parameter() == State::Continue {
            return State::Continue;
        }

        self.start_node_at(checkpoint, SyntaxKind::COMPARISON_CONDITION_NODE);
        self.skip_whitespace();

        if let Some(kind) = self.punctuation() {
            self.skip_whitespace();
            match kind {
                PunctuationKind::Comparison => {
                    self.start_node_at(checkpoint, SyntaxKind::BINARY_COMPARISON_NODE);
                    self.singular_literal();
                    self.finish_node();
                }
                PunctuationKind::In => {
                    self.start_node_at(checkpoint, SyntaxKind::LIST_COMPARISON_NODE);
                    self.list_literal();
                    self.finish_node();
                }
                PunctuationKind::Date => {
                    self.start_node_at(checkpoint, SyntaxKind::DATE_COMPARISON_NODE);
                    self.date_literal();
                    self.finish_node();
                }
            };
        }

        self.finish_node();
        State::Stop
    }

    pub fn punctuation(&mut self) -> Option<PunctuationKind> {
        match self.peek_str().to_lowercase().as_str() {
            "=" => {
                self.start_node(SyntaxKind::EQUAL_OPERATOR_NODE);
                self.bump();
                self.finish_node();
                return Some(PunctuationKind::Comparison);
            }
            "!" => {
                let checkpoint = self.checkpoint();
                self.bump();
                if self.peek_str() == "=" {
                    self.start_node_at(checkpoint, SyntaxKind::NOT_EQUAL_OPERATOR_NODE);
                    self.bump();
                    self.finish_node();
                    return Some(PunctuationKind::Comparison);
                }

                self.start_node_at(checkpoint, SyntaxKind::ERROR);
                self.finish_node();
                return None;
            }
            comparison @ ("<" | ">") => {
                let checkpoint = self.checkpoint();
                self.bump();
                if self.peek_str() == "=" {
                    if comparison == "<" {
                        self.start_node_at(checkpoint, SyntaxKind::LESS_THAN_OR_EQUAL_OPERATOR_NODE)
                    } else {
                        self.start_node_at(
                            checkpoint,
                            SyntaxKind::GREATER_THAN_OR_EQUAL_OPERATOR_NODE,
                        )
                    };
                    self.bump();
                    self.finish_node();
                    return Some(PunctuationKind::Comparison);
                }

                if comparison == "<" {
                    self.start_node_at(checkpoint, SyntaxKind::LESS_THAN_OPERATOR_NODE);
                } else {
                    self.start_node_at(checkpoint, SyntaxKind::GREATER_THAN_OPERATOR_NODE);
                };
                self.finish_node();
                return Some(PunctuationKind::Comparison);
            }
            "in" => {
                self.start_node(SyntaxKind::IN_OPERATOR_NODE);
                self.bump();
                self.finish_node();
                return Some(PunctuationKind::In);
            }
            "after" => {
                self.start_node(SyntaxKind::AFTER_OPERATOR_NODE);
                self.bump();
                self.finish_node();
                return Some(PunctuationKind::Date);
            }
            "before" => {
                self.start_node(SyntaxKind::BEFORE_OPERATOR_NODE);
                self.bump();
                self.finish_node();
                return Some(PunctuationKind::Date);
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PunctuationKind {
    Comparison,
    In,
    Date,
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
    fn comparison() {
        assert_snapshot!(binary_condition("Q123 = 1"));
        assert_snapshot!(binary_condition("Q123 != 1"));
        assert_snapshot!(binary_condition("Q123 < 1"));
        assert_snapshot!(binary_condition("Q123 > 1"));
        assert_snapshot!(binary_condition("Q123 <= 1"));
        assert_snapshot!(binary_condition("Q123 >= 1"));
    }

    #[test]
    fn group() {
        assert_snapshot!(group_condition("(Q123 = 1)"));
        assert_snapshot!(group_condition("((Q123 = 1))"));
    }

    #[test]
    fn logical() {
        assert_snapshot!(logical_condition("Q123 = 1 and Q456 = 2"));
        assert_snapshot!(logical_condition("Q123 = 1 and Q456 = 2 or Q789 = 3"));
    }

    #[test]
    fn complex() {
        assert_snapshot!(logical_condition("(Q123 = 1) and Q456 = 2"));
        assert_snapshot!(logical_condition("Q123 = 1 and (Q456 = 2)"));
        assert_snapshot!(logical_condition("(Q123 = 1) and (Q456 = 2)"));
        assert_snapshot!(logical_condition("(Q123 = 1 and Q456 = 2)"));
        assert_snapshot!(logical_condition("(Q123 = 1 and Q456 = 2) or Q789 = 3"));
        assert_snapshot!(logical_condition("Q123 = 1 and (Q456 = 2 or Q789 = 3)"));
    }
}
