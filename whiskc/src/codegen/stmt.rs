use wsk_vm::Inst;

use crate::ast_resolved::nodes::{
    expr::{ExprKind, IdentExpr},
    stmt::{AssignStmt, Block, ExprStmt, IfStmt, LetStmt, ReturnStmt, Stmt},
};

use super::Codegen;

impl Codegen for Stmt {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        match self {
            Stmt::Block(stmt) => stmt.codegen(ctx),
            Stmt::Expr(stmt) => stmt.codegen(ctx),
            Stmt::Assign(stmt) => stmt.codegen(ctx),
            Stmt::Let(stmt) => stmt.codegen(ctx),
            Stmt::If(stmt) => stmt.codegen(ctx),
            Stmt::Return(stmt) => stmt.codegen(ctx),
        }
    }
}

impl Codegen for Block {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        ctx.push_bound();

        for stmt in &self.stmts {
            stmt.codegen(ctx)?;
        }

        ctx.pop_bound();
        Ok(())
    }
}

impl Codegen for ExprStmt {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        self.expr.codegen(ctx)
    }
}

impl Codegen for AssignStmt {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        self.value.codegen(ctx)?;

        // dont evaluate identifier
        // self.target.codegen(ctx)?;

        if let ExprKind::Identifier(IdentExpr { sym_id, .. }) = self.target.get_kind() {
            let id = ctx.get_local(*sym_id);
            ctx.get_current_fi_mut().push_inst(Inst::Store(id));
            Ok(())
        } else {
            unimplemented!("unsupported assignment type")
        }
    }
}

impl Codegen for LetStmt {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        self.value.codegen(ctx)?;

        let id = ctx.get_local(self.sym_id);
        ctx.get_current_fi_mut().push_inst(Inst::Store(id));

        Ok(())
    }
}

impl Codegen for IfStmt {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        self.cond.codegen(ctx)?;

        let then_insert_point = ctx.get_current_fi_mut().len();

        self.body.codegen(ctx)?;

        let merge_insert_point = ctx.get_current_fi_mut().len();

        if let Some(body) = &self.else_body {
            body.codegen(ctx)?;
        }

        let func = ctx.get_current_fi_mut();

        if !matches!(self.body.stmts[..], [.., Stmt::Return(_)]) {
            let jmp_dist = func.len() - merge_insert_point + 1;
            func.insert_inst(merge_insert_point, Inst::Jmp(jmp_dist as isize));
        }

        let jmp_dist = func.len() - then_insert_point + 1;
        func.insert_inst(then_insert_point, Inst::JmpFalse(jmp_dist as isize));

        Ok(())
    }
}

impl Codegen for ReturnStmt {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        if let Some(expr) = &self.expr {
            expr.codegen(ctx)?;
        }
        ctx.get_current_fi_mut().push_inst(Inst::Ret);
        Ok(())
    }
}
