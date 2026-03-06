// The Halo Programming Language
// Version: 0.1.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::fmt;

// ===== Position =====
// Represents a position in the source code for error reporting
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

// ===== Block =====
// Represents a block of statements with position information
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub pos: Position,
}

impl Block {
    pub fn new(stmts: Vec<Statement>, pos: Position) -> Self {
        Block { stmts, pos }
    }
}

// Type enum removed - no explicit types

// ===== BinOp =====
// Binary operators supported in expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinOp::*;
        let s = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Eq => "==",
            Neq => "!=",
            Lt => "<",
            Gt => ">",
            Le => "<=",
            Ge => ">=",
            And => "&&",
            Or => "||",
        };
        write!(f, "{}", s)
    }
}

// ===== Expression =====
// Language expressions
#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64, Position),
    Float(f64, Position),
    Bool(bool, Position),
    Var(String, Position),
    Unary {
        operator: String,
        expr: Box<Expression>,
        pos: Position,
    },
    Binary {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
        pos: Position,
    },
    Assign {
        name: String,
        value: Box<Expression>,
        pos: Position,
    },
    Call {
        name: String,
        args: Vec<Expression>,
        pos: Position,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n, _) => write!(f, "{}", n),
            Expression::Float(n, _) => write!(f, "{}", n),
            Expression::Bool(b, _) => write!(f, "{}", b),
            Expression::Var(name, _) => write!(f, "{}", name),
            Expression::Unary { operator, expr, .. } => write!(f, "{}{}", operator, expr),
            Expression::Binary {
                left, op, right, ..
            } => write!(f, "({} {} {})", left, op, right),
            Expression::Assign { name, value, .. } => write!(f, "{} = {}", name, value),
            Expression::Call { name, args, .. } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl Expression {
    pub fn pos(&self) -> Position {
        match self {
            Expression::Number(_, p) => *p,
            Expression::Float(_, p) => *p,
            Expression::Bool(_, p) => *p,
            Expression::Var(_, p) => *p,
            Expression::Unary { pos, .. } => *pos,
            Expression::Binary { pos, .. } => *pos,
            Expression::Assign { pos, .. } => *pos,
            Expression::Call { pos, .. } => *pos,
        }
    }
}

// ===== Statement =====
// Language statements
#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expression),
    VarDecl {
        name: String,
        init: Option<Expression>,
        pos: Position,
    },
    If {
        cond: Expression,
        then_branch: Block,
        else_branch: Option<Block>,
        pos: Position,
    },
    While {
        cond: Expression,
        body: Block,
        pos: Position,
    },
    Return {
        value: Option<Expression>,
        pos: Position,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expr(e) => write!(f, "{}", e),
            Statement::VarDecl { name, init, .. } => {
                write!(f, "{} ", name)?;
                if let Some(init) = init {
                    write!(f, "= {}", init)?;
                }
                Ok(())
            }
            Statement::If {
                cond,
                then_branch,
                else_branch,
                ..
            } => {
                write!(f, "if {} {{", cond)?;
                for stmt in &then_branch.stmts {
                    write!(f, "\n    {}", stmt)?;
                }
                if let Some(else_b) = else_branch {
                    write!(f, "\n}} else {{")?;
                    for stmt in &else_b.stmts {
                        write!(f, "\n    {}", stmt)?;
                    }
                }
                write!(f, "\n}}")
            }
            Statement::While { cond, body, .. } => {
                write!(f, "while {} {{", cond)?;
                for stmt in &body.stmts {
                    write!(f, "\n    {}", stmt)?;
                }
                write!(f, "\n}}")
            }
            Statement::Return { value, .. } => {
                write!(f, "return")?;
                if let Some(v) = value {
                    write!(f, " {}", v)?;
                }
                Ok(())
            }
        }
    }
}

impl Statement {
    pub fn pos(&self) -> Position {
        match self {
            Statement::Expr(e) => e.pos(),
            Statement::VarDecl { pos, .. } => *pos,
            Statement::If { pos, .. } => *pos,
            Statement::While { pos, .. } => *pos,
            Statement::Return { pos, .. } => *pos,
        }
    }
}

// ===== TopLevel =====
// Top-level declarations: functions and global variables
#[derive(Debug, Clone)]
pub enum TopLevel {
    Function {
        name: String,
        params: Vec<String>,
        body: Block,
        pos: Position,
    },
    GlobalVar {
        name: String,
        init: Option<Expression>,
        pos: Position,
    },
}

impl TopLevel {
    pub fn pos(&self) -> Position {
        match self {
            TopLevel::Function { pos, .. } => *pos,
            TopLevel::GlobalVar { pos, .. } => *pos,
        }
    }
}

// ===== Program =====
// Root of the AST: complete program
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<TopLevel>,
}

impl Program {
    pub fn new(items: Vec<TopLevel>) -> Self {
        Program { items }
    }
}
