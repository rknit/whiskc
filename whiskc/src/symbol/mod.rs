mod common;
mod symbol_id;
mod symbol_table;
pub mod ty;

pub use symbol_id::*;
pub use symbol_table::SymbolTable;

#[derive(Debug, Clone)]
pub struct TypeSymbol {
    id: TypeId,
    pub name: String,
}
impl TypeSymbol {
    pub fn get_id(&self) -> TypeId {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct FuncSymbol {
    id: FuncId,
    pub name: String,
    pub params: Vec<VarId>,
    pub ret_ty: TypeId,
    pub entry_block: BlockId,
}
impl FuncSymbol {
    pub fn get_id(&self) -> FuncId {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct BlockSymbol {
    id: BlockId,
    pub func: FuncId,
    pub parent_block: Option<BlockId>,
}
impl BlockSymbol {
    pub fn get_id(&self) -> BlockId {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct VarSymbol {
    id: VarId,
    pub block: BlockId,
    pub name: String,
    pub ty: TypeId,
}
impl VarSymbol {
    pub fn get_id(&self) -> VarId {
        self.id
    }
}
