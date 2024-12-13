use crate::{
    ast::AST,
    ast_resolved::{
        errors::ResolveError,
        nodes::{ast::ResolvedAST, item::Item},
        Resolve, ResolveContext,
    },
    symbol_table::SymbolTable,
};

pub fn resolve(ast: &AST) -> Result<(ResolvedAST, SymbolTable), Vec<ResolveError>> {
    let mut global_table = SymbolTable::new();
    let mut ctx = ResolveContext::new(&mut global_table);

    for item in &ast.items {
        use crate::ast::nodes::item::Item;
        match item {
            Item::Function(function) => function.sig.resolve(&mut ctx),
            Item::ExternFunction(function) => function.sig.resolve(&mut ctx),
        };
    }

    let mut items = Vec::new();

    for item in &ast.items {
        let Some(item): Option<Item> = ({
            use crate::ast::nodes::item::Item;
            match item {
                Item::Function(function) => function.resolve(&mut ctx).map(|v| v.into()),
                Item::ExternFunction(function) => function.resolve(&mut ctx).map(|v| v.into()),
            }
        }) else {
            continue;
        };
        items.push(item);
    }

    ctx.finalize()?;
    Ok((ResolvedAST { items }, global_table))
}