use crate::{ast::location::Located, symbol_table::SymbolID, ty::Type};

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(ExprStmt),
    Let(LetStmt),
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub sym_id: SymbolID,
    pub name: Located<String>,
    pub ty: Type,
    pub value: Expr,
}
