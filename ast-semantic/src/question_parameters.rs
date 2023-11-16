use crate::{SemanticOption, SemanticResult};
use ast::{Program, QuestionParameter, Visitor};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct QuestionParameterRule {
    parameters: HashMap<u64, QuestionParameter>,
}

#[derive(Debug)]
pub struct QuestionParametersRuleOption<'rule> {
    pub questions: HashSet<u64>,
    pub node: &'rule Program,
}

impl Visitor for QuestionParameterRule {
    fn visit_question_parameter(&mut self, question_parameter: &ast::QuestionParameter) {
        self.parameters
            .insert(question_parameter.id().value(), question_parameter.clone());
    }
}

impl<'rule> From<&'rule SemanticOption<'rule>> for QuestionParametersRuleOption<'rule> {
    fn from(option: &'rule SemanticOption) -> Self {
        let mut questions = HashSet::new();
        for question in option.questions.values() {
            questions.insert(question.id.0);
        }
        let node = option.node;
        Self { questions, node }
    }
}

impl QuestionParameterRule {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
        }
    }
}

pub fn check(option: QuestionParametersRuleOption) -> SemanticResult {
    let mut visitor = QuestionParameterRule::new();
    visitor.visit_program(option.node);
    for parameter in visitor.parameters {
        if !option.questions.contains(&parameter.0) {
            invalidate!("question parameter {} is not a question", parameter.0);
        }
    }

    Ok(())
}
