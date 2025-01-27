use std::{fs, path::PathBuf};

use crate::{
    ast::{self, AST},
    ast_resolved::{self, passes::run_passes, ResolvedAST},
    codegen::codegen_wsk_vm,
};

#[derive(Debug)]
pub struct Module {
    name: String,
    path: PathBuf,
    ast: Option<AST>,
    resolved_ast: Option<ResolvedAST>,
}
impl Module {
    pub fn new(path: PathBuf) -> Self {
        Self {
            name: path
                .file_stem()
                .expect("file name")
                .to_str()
                .expect("valid string")
                .to_string(),
            path,
            ast: None,
            resolved_ast: None,
        }
    }

    pub fn parse_ast(&mut self) -> Option<&AST> {
        self.ast = match ast::parse(&self.path) {
            Ok(ast) => Some(ast),
            Err(errors) => {
                dbg!(&errors);
                return None;
            }
        };
        self.ast.as_ref()
    }

    pub fn resolve_ast(&mut self) -> Option<&ResolvedAST> {
        let Some(ast) = &self.ast else {
            return None;
        };
        self.resolved_ast = match ast_resolved::resolve(ast) {
            Ok(ast) => Some(ast),
            Err(errs) => {
                dbg!(&errs);
                return None;
            }
        };
        self.resolved_ast.as_ref()
    }

    pub fn run_passes(&mut self) -> Option<&ResolvedAST> {
        let Some(ast) = &mut self.resolved_ast else {
            return None;
        };
        run_passes(ast);
        self.resolved_ast.as_ref()
    }

    pub fn codegen(&self) {
        let Some(ast) = &self.resolved_ast else {
            return;
        };

        let prog = match codegen_wsk_vm(ast) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };

        let mut bin_path = self.path.clone();
        bin_path.set_extension("wc");

        println!("wrote binary to {}", bin_path.display());
        let bin = prog.to_bin();
        fs::write(bin_path, bin).unwrap();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
