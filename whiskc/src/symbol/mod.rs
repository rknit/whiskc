#![allow(dead_code)]

mod symbol_table;

pub use symbol_table::SymbolTable;

use self::symbol_table::{Block, BlockId, FuncId, Function, Param, VarId, Variable};

pub struct FuncSymbol<'a> {
    table: &'a mut SymbolTable,
    id: FuncId,
}
impl<'a> FuncSymbol<'a> {
    pub fn new(table: &'a mut SymbolTable, id: FuncId) -> Self {
        Self { table, id }
    }

    fn get(&self) -> &Function {
        self.table.funcs.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Function {
        self.table.funcs.get_mut(&self.id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.get().name
    }

    pub fn set_param_name(&mut self, index: usize, name: String) -> Option<&mut Self> {
        self.get_mut().params.get_mut(index)?.name = name;
        Some(self)
    }

    pub fn get_param(&self, index: usize) -> Option<&Param> {
        self.get().params.get(index)
    }
}

pub struct BlockSymbol<'a> {
    table: &'a mut SymbolTable,
    id: BlockId,
}
impl<'a> BlockSymbol<'a> {
    pub fn new(table: &'a mut SymbolTable, id: BlockId) -> Self {
        Self { table, id }
    }

    fn get(&self) -> &Block {
        self.table.blocks.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Block {
        self.table.blocks.get_mut(&self.id).unwrap()
    }

    pub fn set_parent_block(&mut self, block: BlockId) -> &mut Self {
        assert!(
            self.id == block,
            "cannot assign the block itself as its parent block"
        );
        self.get_mut().parent_block = Some(block);
        self
    }

    pub fn get_function(&self) -> FuncId {
        self.get().func
    }

    pub fn get_id(&self) -> BlockId {
        self.id
    }
}

pub struct VarSymbol<'a> {
    table: &'a mut SymbolTable,
    id: VarId,
}
impl<'a> VarSymbol<'a> {
    pub fn new(table: &'a mut SymbolTable, id: VarId) -> Self {
        Self { table, id }
    }

    fn get(&self) -> &Variable {
        self.table.vars.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Variable {
        self.table.vars.get_mut(&self.id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.get().name
    }

    pub fn get_block(&self) -> BlockId {
        self.get().block
    }
}
