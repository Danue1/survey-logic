#[macro_use]
pub mod error;
mod question_parameters;

use ast::Program;
use entity::{Question, QuestionId};
pub use error::*;
use std::collections::HashMap;

pub struct SemanticOption<'semantic> {
    pub questions: &'semantic HashMap<QuestionId, Question>,
    pub node: &'semantic Program,
}

pub fn check(option: &SemanticOption) -> SemanticResult {
    question_parameters::check(option.into())?;

    Ok(())
}
