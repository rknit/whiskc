use crate::ast::parsing::token::Operator;

use super::nodes::expr::{BinaryExpr, ExprKind, UnaryExpr};

pub trait EvalConstant {
    fn eval_constant(&self) -> Option<ExprKind>;
}

impl EvalConstant for ExprKind {
    fn eval_constant(&self) -> Option<ExprKind> {
        match self {
            ExprKind::Integer(_)
            | ExprKind::Bool(_)
            | ExprKind::Identifier(_)
            | ExprKind::Call(_) => None,
            ExprKind::Unary(expr) => expr.eval_constant(),
            ExprKind::Binary(expr) => expr.eval_constant(),
        }
    }
}

impl EvalConstant for UnaryExpr {
    fn eval_constant(&self) -> Option<ExprKind> {
        match self.op {
            Operator::Sub => match self.expr.get_kind() {
                ExprKind::Integer(v) => Some(ExprKind::Integer(-v)),
                _ => None,
            },
            Operator::Not => match self.expr.get_kind() {
                ExprKind::Bool(v) => Some(ExprKind::Bool(!v)),
                _ => None,
            },
            _ => unimplemented!("EvalConstant unary"),
        }
    }
}

impl EvalConstant for BinaryExpr {
    fn eval_constant(&self) -> Option<ExprKind> {
        let get_ints = || match (&self.left.get_kind(), &self.right.get_kind()) {
            (ExprKind::Integer(lhs), ExprKind::Integer(rhs)) => Some((*lhs, *rhs)),
            _ => None,
        };
        let get_bools = || match (&self.left.get_kind(), &self.right.get_kind()) {
            (ExprKind::Bool(lhs), ExprKind::Bool(rhs)) => Some((*lhs, *rhs)),
            _ => None,
        };
        Some(match self.op {
            Operator::Add => {
                let (lhs, rhs) = get_ints()?;
                ExprKind::Integer(lhs + rhs)
            }
            Operator::Sub => {
                let (lhs, rhs) = get_ints()?;
                ExprKind::Integer(lhs - rhs)
            }
            Operator::And => {
                let (lhs, rhs) = get_bools()?;
                ExprKind::Bool(lhs && rhs)
            }
            Operator::Or => {
                let (lhs, rhs) = get_bools()?;
                ExprKind::Bool(lhs || rhs)
            }
            Operator::Equal => ExprKind::Bool(if let Some((lhs, rhs)) = get_ints() {
                lhs == rhs
            } else if let Some((lhs, rhs)) = get_bools() {
                lhs == rhs
            } else {
                return None;
            }),
            Operator::NotEqual => ExprKind::Bool(if let Some((lhs, rhs)) = get_ints() {
                lhs != rhs
            } else if let Some((lhs, rhs)) = get_bools() {
                lhs != rhs
            } else {
                return None;
            }),
            Operator::Less => {
                let (lhs, rhs) = get_ints()?;
                ExprKind::Bool(lhs < rhs)
            }
            Operator::LessEqual => {
                let (lhs, rhs) = get_ints()?;
                ExprKind::Bool(lhs <= rhs)
            }
            Operator::Greater => {
                let (lhs, rhs) = get_ints()?;
                ExprKind::Bool(lhs > rhs)
            }
            Operator::GreaterEqual => {
                let (lhs, rhs) = get_ints()?;
                ExprKind::Bool(lhs >= rhs)
            }
            _ => unimplemented!("EvalConstant binary"),
        })
    }
}