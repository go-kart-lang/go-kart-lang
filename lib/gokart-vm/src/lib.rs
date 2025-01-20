mod ops;
mod vm;
mod jit;

pub use vm::VM;

pub use jit::Optimization;
pub use jit::TailCallOptimization;
pub use jit::DeadCodeElimination;
pub use jit::ConstantFolding;
