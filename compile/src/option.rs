use entity::{Question, QuestionId};
use std::collections::HashMap;

#[derive(Debug)]
#[non_exhaustive]
pub struct CompileOption {
    pub questions: HashMap<QuestionId, Question>,
}

impl CompileOption {
    pub const fn new(questions: HashMap<QuestionId, Question>) -> Self {
        CompileOption { questions }
    }
}
