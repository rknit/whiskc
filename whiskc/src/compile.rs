use std::{fs, path::PathBuf};

use crate::{ast, codegen::codegen_wsk_vm, lowering};

#[derive(Default)]
pub struct CompileSwitch {
    pub do_parse_ast: bool,
    pub debug_ast: bool,
    pub do_resolve_module: bool,
    pub debug_module: bool,
    pub do_codegen: bool,
}

pub fn compile(source_path: PathBuf, switches: CompileSwitch) {
    if !switches.do_parse_ast {
        return;
    }
    let ast = match ast::parse(&source_path) {
        Ok(ast) => ast,
        Err(errors) => {
            dbg!(&errors);
            return;
        }
    };
    if switches.debug_ast {
        dbg!(&ast);
    }

    if !switches.do_resolve_module {
        return;
    }
    let module = match lowering::resolve(&ast) {
        Ok(module) => module,
        Err(errs) => {
            dbg!(&errs);
            return;
        }
    };
    if switches.debug_module {
        dbg!(&module);
    }

    if !switches.do_codegen {
        return;
    }
    let prog = match codegen_wsk_vm(&module) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    let mut out_path = source_path.clone();
    out_path.set_extension("wc");
    println!("wrote binary to {}", out_path.display());
    fs::write(out_path, prog.to_bin()).unwrap();
}
