use crate::SyntaxResult;
use ast::Program;
use syntax::ProgramNode;

pub fn lower_program(node: ProgramNode) -> SyntaxResult<Program> {
    let mut statements = vec![];
    for statement in node.statements() {
        statements.push(crate::lower_statement(statement)?);
    }
    Ok(Program::new(node.syntax(), statements))
}
