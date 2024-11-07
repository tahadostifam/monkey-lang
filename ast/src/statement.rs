use std::fmt;
use token::{Span, Token};

use crate::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(Variable),
    Expression(Expression),
    If(If),
    Return(Return),
}

pub fn format_statements(stmts: &Vec<Statement>) -> String {
    stmts.iter().map(|stmt| stmt.to_string()).collect()
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub identifier: Token,
    pub expr: Expression,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expression,
    pub consequent: Box<BlockStatement>,
    pub alternate: Option<Box<BlockStatement>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub argument: Expression,
    pub span: Span,
}
