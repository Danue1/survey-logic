pub type SyntaxNode = rowan::SyntaxNode<SyntaxKind>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    EOF,
    UNKNOWN,

    // for Tokens

    // Punctuation
    LEFT_PAREN,    // (
    RIGHT_PAREN,   // )
    LEFT_BRACKET,  // [
    RIGHT_BRACKET, // ]
    LEFT_CHEVRON,  // <
    RIGHT_CHEVRON, // >
    EQUAL,         // =
    EXCLAMATION,   // !
    DOT,           // .
    COMMA,         // ,
    NUMERIC,
    STRING,
    IDENTIFIER,
    WHITESPACE,

    // for Nodes
    ERROR,

    // Program
    // = _ (Statement | _)*
    PROGRAM_NODE,

    // Statement
    // = LogicalCondition
    STATEMENT_NODE,

    // LogicalCondition
    // = AtomicCondition _ "AND" _ AtomicCondition (_ LogicalCondition)*
    // | AtomicCondition _ "OR" _ AtomicCondition (_ LogicalCondition)*
    LOGICAL_CONDITION_NODE,

    // AtomicCondition
    // = GroupCondition
    // | ComparisonCondition

    // GroupCondition
    // = "(" _ Condition _ ")
    // | LogicalCondition
    GROUP_CONDITION_NODE,

    // ComparisonCondition
    // = BinaryComparison
    // | ListComparison
    // | DateComparison
    COMPARISON_CONDITION_NODE,

    // BinaryComparison
    // = QuestionParameter _ "=" _ SingularLiteral
    // | QuestionParameter _ "!=" _ SingularLiteral
    // | QuestionParameter _ "<" _ SingularLiteral
    // | QuestionParameter _ ">" _ SingularLiteral
    // | QuestionParameter _ "<=" _ SingularLiteral
    // | QuestionParameter _ ">=" _ SingularLiteral
    BINARY_COMPARISON_NODE,

    // ListComparison
    // = QuestionParameter _ "IN" _ ListLiteral
    LIST_COMPARISON_NODE,

    // DateComparison
    // = QuestionParameter _ "AFTER" _ DateLiteral
    // | QuestionParameter _ "BEFORE" _ DateLiteral
    DATE_COMPARISON_NODE,

    // QuestionParameter
    // = "Q" NumericLiteral
    QUESTION_PARAMETER_NODE,

    // SingularLiteral
    // = NumericLiteral
    // | StringLiteral

    // ListLiteral
    // = "[" _ (SingularLiteral _ ("," _ SingularLiteral)*)? _ "]"
    LIST_LITERAL_NODE,

    // NumericLiteral
    // = _ [0-9]+ _
    NUMERIC_LITERAL_NODE,

    // DateLiteral
    // = StringLiteral
    DATE_LITERAL_NODE,

    // StringLiteral
    // = _ "\"" [^"]* "\"" _
    // | _ "'" [^']* "'" _
    STRING_LITERAL_NODE,

    // Punctuation
    EQUAL_OPERATOR_NODE,                 // =
    NOT_EQUAL_OPERATOR_NODE,             // !=
    LESS_THAN_OPERATOR_NODE,             // <
    GREATER_THAN_OPERATOR_NODE,          // >
    LESS_THAN_OR_EQUAL_OPERATOR_NODE,    // <=
    GREATER_THAN_OR_EQUAL_OPERATOR_NODE, // >=
    IN_OPERATOR_NODE,                    // in
    AFTER_OPERATOR_NODE,                 // after
    BEFORE_OPERATOR_NODE,                // before
    AND_OPERATOR_NODE,                   // and
    OR_OPERATOR_NODE,                    // or
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl rowan::Language for SyntaxKind {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
