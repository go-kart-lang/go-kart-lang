mod gc;
mod heap;
mod ops;
mod state;
mod value;
mod vm;
mod jit;

pub use gc::GC;
pub use vm::VM;

pub use jit::Optimization;
pub use jit::TailCallOptimization;
pub use jit::DeadCodeElimination;
pub use jit::ConstantFolding;
