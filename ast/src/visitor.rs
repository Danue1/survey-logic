#[allow(unused_variables)]
pub trait Visitor: Sized {
    fn visit_program(&mut self, program: &crate::Program) {
        visit_program(self, program);
    }

    fn visit_statement(&mut self, statement: &crate::Statement) {
        visit_statement(self, statement);
    }

    fn visit_condition(&mut self, condition: &crate::Condition) {
        visit_condition(self, condition);
    }

    fn visit_group_condition(&mut self, group_condition: &crate::GroupCondition) {
        visit_group_condition(self, group_condition)
    }

    fn visit_logical_condition(&mut self, logical_condition: &crate::LogicalCondition) {
        visit_logical_condition(self, logical_condition);
    }

    fn visit_comparison_condition(&mut self, comparison_condition: &crate::ComparisonCondition) {
        visit_comparison_condition(self, comparison_condition);
    }

    fn visit_logical_condition_operator(
        &mut self,
        logical_condition_operator: &crate::LogicalConditionOperator,
    ) {
        //
    }

    fn visit_comparison(&mut self, comparison: &crate::Comparison) {
        visit_comparison(self, comparison);
    }

    fn visit_binary_comparison(&mut self, binary_comparison: &crate::BinaryComparison) {
        visit_binary_comparison(self, binary_comparison);
    }

    fn visit_list_comparison(&mut self, list_comparison: &crate::ListComparison) {
        visit_list_comparison(self, list_comparison);
    }

    fn visit_date_comparison(&mut self, date_comparison: &crate::DateComparison) {
        visit_date_comparison(self, date_comparison);
    }

    fn visit_question_parameter(&mut self, question_parameter: &crate::QuestionParameter) {
        visit_question_parameter(self, question_parameter);
    }

    fn visit_binary_comparison_operator(
        &mut self,
        binary_comparison_operator: &crate::BinaryComparisonOperator,
    ) {
        //
    }

    fn visit_list_literal(&mut self, list_literal: &crate::ListLiteral) {
        visit_list_literal(self, list_literal);
    }

    fn visit_singular_literal(&mut self, singular_literal: &crate::SingularLiteral) {
        visit_singular_literal(self, singular_literal);
    }

    fn visit_numeric_literal(&mut self, numeric_literal: &crate::NumericLiteral) {
        //
    }

    fn visit_string_literal(&mut self, string_literal: &crate::StringLiteral) {
        //
    }

    fn visit_date_literal(&mut self, date_literal: &crate::DateLiteral) {
        //
    }
}

pub fn visit_program<V: Visitor>(visitor: &mut V, program: &crate::Program) {
    for statement in program.statements() {
        visitor.visit_statement(statement);
    }
}

pub fn visit_statement<V: Visitor>(visitor: &mut V, statement: &crate::Statement) {
    visitor.visit_condition(statement.condition());
}

pub fn visit_condition<V: Visitor>(visitor: &mut V, condition: &crate::Condition) {
    match condition.kind() {
        crate::ConditionKind::Group(group) => visitor.visit_group_condition(group),
        crate::ConditionKind::Logical(logical) => visitor.visit_logical_condition(logical),
        crate::ConditionKind::Binary(binary) => visitor.visit_comparison_condition(binary),
    }
}

pub fn visit_group_condition<V: Visitor>(visitor: &mut V, group_condition: &crate::GroupCondition) {
    visitor.visit_condition(group_condition.condition());
}

pub fn visit_logical_condition<V: Visitor>(
    visitor: &mut V,
    logical_condition: &crate::LogicalCondition,
) {
    visitor.visit_condition(logical_condition.left());
    visitor.visit_logical_condition_operator(logical_condition.operator());
    visitor.visit_condition(logical_condition.right());
}

pub fn visit_comparison_condition<V: Visitor>(
    visitor: &mut V,
    comparison_condition: &crate::ComparisonCondition,
) {
    visitor.visit_comparison(comparison_condition.comparison());
}

pub fn visit_comparison<V: Visitor>(visitor: &mut V, comparison: &crate::Comparison) {
    match comparison.kind() {
        crate::ComparisonKind::Binary(binary) => visitor.visit_binary_comparison(binary),
        crate::ComparisonKind::List(list) => visitor.visit_list_comparison(list),
        crate::ComparisonKind::Date(date) => visitor.visit_date_comparison(date),
    }
}

pub fn visit_binary_comparison<V: Visitor>(
    visitor: &mut V,
    binary_comparison: &crate::BinaryComparison,
) {
    visitor.visit_question_parameter(binary_comparison.parameter());
    visitor.visit_binary_comparison_operator(binary_comparison.operator());
    visitor.visit_singular_literal(binary_comparison.value());
}

pub fn visit_list_comparison<V: Visitor>(visitor: &mut V, list_comparison: &crate::ListComparison) {
    visitor.visit_question_parameter(list_comparison.parameter());
    visitor.visit_list_literal(list_comparison.list());
}

pub fn visit_date_comparison<V: Visitor>(visitor: &mut V, date_comparison: &crate::DateComparison) {
    visitor.visit_question_parameter(date_comparison.parameter());
    visitor.visit_date_literal(date_comparison.value());
}

pub fn visit_question_parameter<V: Visitor>(
    visitor: &mut V,
    question_parameter: &crate::QuestionParameter,
) {
    visitor.visit_numeric_literal(question_parameter.id());
}

pub fn visit_list_literal<V: Visitor>(visitor: &mut V, list_literal: &crate::ListLiteral) {
    for value in list_literal.values() {
        visitor.visit_singular_literal(value);
    }
}

pub fn visit_singular_literal<V: Visitor>(
    visitor: &mut V,
    singular_literal: &crate::SingularLiteral,
) {
    match singular_literal.kind() {
        crate::SingularLiteralKind::Numeric(numeric) => visitor.visit_numeric_literal(numeric),
        crate::SingularLiteralKind::String(string) => visitor.visit_string_literal(string),
    }
}
