pub mod inst;
pub mod inst_code;
pub mod value;
pub mod vm;

pub use inst::{Cmp, Inst, RunError};
pub use value::Value;
pub use vm::{VMError, VM};
