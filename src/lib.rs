#![feature(associated_type_defaults)]
pub mod memtable;
pub mod normalized_regs;
pub mod process;
pub mod syscall_table;
pub mod trace;
pub mod type_parser;

pub type BoxedError = Box<dyn std::error::Error>;
