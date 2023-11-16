use crate::context::{Context, State};

impl Context {
    pub fn statement(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if self.logical_condition() == State::Continue {
            return State::Continue;
        }

        self.start_node_at(checkpoint, syntax_kind::SyntaxKind::STATEMENT_NODE);
        self.finish_node();
        State::Stop
    }
}
