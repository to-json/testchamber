// Keeping this list of features around because I meant to use a few of them
// #![feature(type_alias_impl_trait)]
// #![feature(return_position_impl_trait_in_trait)]
// #![feature(impl_trait_in_assoc_type)]
// #![feature(trait_alias)]

use clap::Parser;
use testchamber::memtable::MetaMemoryTable;
use testchamber::normalized_regs::{I64Regs, Syscall};
use testchamber::process::Process;
use testchamber::syscall_table::SyscallTable;
use testchamber::trace::trace;
use testchamber::BoxedError;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(trailing_var_arg(true), required(true))]
    command: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    /// toggle color, defaults to True
    color: bool,

    #[arg(short, long, default_value("./syscall.json"))]
    syscall_table_path: String,
}

fn main() -> Result<(), BoxedError> {
    let cli = Cli::parse();
    let syscall_table = SyscallTable::new(cli.syscall_table_path)?;
    let print_syscall = I64Regs::printer(syscall_table)?;
    let mut memory_table = MetaMemoryTable::new();
    let (executable, args) = cli.command.split_first().unwrap();
    let mut cmd = Process::new(executable.to_string(), Some(args.into()));
    cmd.build_command();
    trace::<I64Regs>(&mut cmd, &mut memory_table, print_syscall).map_err(|e| e.into())
}
