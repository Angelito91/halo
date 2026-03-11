// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::fmt;

// ===== Position =====
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

// ===== Block =====
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
    #[allow(dead_code)]
    pub pos: Position,
}

impl Block {
    pub fn new(stmts: Vec<Statement>, pos: Position) -> Self {
        Block { stmts, pos }
    }
}

// ===== BinOp =====
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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expression {
    Number(i64, Position),
    Float(f64, Position),
    Bool(bool, Position),
    /// String literal: "hello"
    StringLiteral(String, Position),
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
            Expression::StringLiteral(s, _) => write!(f, "\"{}\"", s),
            Expression::Var(name, _) => write!(f, "{}", name),
            Expression::Unary { operator, expr, .. } => write!(f, "{}{}", operator, expr),
            Expression::Binary {
                left, op, right, ..
            } => {
                write!(f, "({} {} {})", left, op, right)
            }
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
            Expression::StringLiteral(_, p) => *p,
            Expression::Var(_, p) => *p,
            Expression::Unary { pos, .. } => *pos,
            Expression::Binary { pos, .. } => *pos,
            Expression::Assign { pos, .. } => *pos,
            Expression::Call { pos, .. } => *pos,
        }
    }
}

// ===== ElseIf branch =====
/// A single `else if <cond> { ... }` clause.
#[derive(Debug, Clone)]
pub struct ElseIfBranch {
    pub cond: Expression,
    pub body: Block,
    pub pos: Position,
}

// ===== Statement =====
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
        /// Zero or more `else if` clauses, in order.
        else_if_branches: Vec<ElseIfBranch>,
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
    /// `break` — exit the nearest enclosing `while` loop.
    Break {
        pos: Position,
    },
    /// `continue` — jump to the condition of the nearest enclosing `while` loop.
    Continue {
        pos: Position,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expr(e) => write!(f, "{}", e),
            Statement::VarDecl { name, init, .. } => {
                write!(f, "{}", name)?;
                if let Some(init) = init {
                    write!(f, " = {}", init)?;
                }
                Ok(())
            }
            Statement::If {
                cond,
                then_branch,
                else_if_branches,
                else_branch,
                ..
            } => {
                write!(f, "if {} {{", cond)?;
                for stmt in &then_branch.stmts {
                    write!(f, "\n    {}", stmt)?;
                }
                for branch in else_if_branches {
                    write!(f, "\n}} else if {} {{", branch.cond)?;
                    for stmt in &branch.body.stmts {
                        write!(f, "\n    {}", stmt)?;
                    }
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
            Statement::Break { .. } => write!(f, "break"),
            Statement::Continue { .. } => write!(f, "continue"),
        }
    }
}

impl Statement {
    #[allow(dead_code)]
    pub fn pos(&self) -> Position {
        match self {
            Statement::Expr(e) => e.pos(),
            Statement::VarDecl { pos, .. } => *pos,
            Statement::If { pos, .. } => *pos,
            Statement::While { pos, .. } => *pos,
            Statement::Return { pos, .. } => *pos,
            Statement::Break { pos } => *pos,
            Statement::Continue { pos } => *pos,
        }
    }
}

// ===== TopLevel =====
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
    /// A bare statement at the top level (if, while, break, continue, return).
    /// The compiler collects these and emits them inside a generated `main`.
    Stmt { stmt: Statement, pos: Position },
}

impl TopLevel {
    #[allow(dead_code)]
    pub fn pos(&self) -> Position {
        match self {
            TopLevel::Function { pos, .. } => *pos,
            TopLevel::GlobalVar { pos, .. } => *pos,
            TopLevel::Stmt { pos, .. } => *pos,
        }
    }
}

// ===== Program =====
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<TopLevel>,
}

impl Program {
    pub fn new(items: Vec<TopLevel>) -> Self {
        Program { items }
    }
}
