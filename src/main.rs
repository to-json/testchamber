#![feature(type_alias_impl_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(associated_type_defaults)]
#![feature(trait_alias)]
use std::io::Error;

use clap::Parser;
use nix::sys::{
    ptrace,
    wait::{waitpid, WaitStatus},
};

mod process;
use process::Process;

mod memtable;
use memtable::{MemLookup, MetaMemoryTable};

mod syscall_table;
use syscall_table::SyscallTable;

mod normalized_regs;
use normalized_regs::I64Regs;
use normalized_regs::Registers;

fn trace<T: Registers>(
    process: &mut Process,
    memory_table: &mut impl MemLookup<Entry = T::MemType>,
    printer: Box<dyn Fn(&T)>,
) -> Result<(), Error> {
    let pre_exec = || -> Result<(), Error> {
        let mut filter = libseccomp::ScmpFilterContext::new(libseccomp::ScmpAction::Allow).unwrap();
        let _ = filter.add_arch(libseccomp::ScmpArch::X8664);
        let _ = filter.load();
        use nix::sys::ptrace::traceme;
        traceme()?;
        Ok(())
    };
    //
    process.pre_exec = Some(pre_exec);
    process.set_pre_exec()?;
    process.spawn();

    // every syscall has an entrance and exit point. in order to only log the
    // syscall once, we toggle a var every loop
    let mut is_sys_exit = false;
    let child_pid = process.pid.unwrap();
    loop {
        ptrace::syscall(child_pid, None)?;
        if is_sys_exit {
            let wp = waitpid(child_pid, None)?;
            match wp {
                WaitStatus::Exited(_, _) => {
                    println!("exited");
                    break;
                }
                _ => {
                    let regs = ptrace::getregs(child_pid)?;
                    let normalized_regs = T::from_regs(&regs, memory_table);
                    printer(&normalized_regs);
                }
            }
        } else {
            waitpid(child_pid, None)?;
        }

        is_sys_exit = !is_sys_exit;
    }
    Ok(())
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let syscall_table = SyscallTable::new(cli.syscall_table_path)?;
    let print_syscall = I64Regs::printer(syscall_table)?;
    let mut memory_table = MetaMemoryTable::new();
    let (executable, args) = cli.command.split_first().unwrap();
    let mut cmd = Process::new(executable.to_string(), Some(args.into()));
    cmd.build_command();
    trace::<I64Regs>(&mut cmd, &mut memory_table, print_syscall).map_err(|e| e.into())
}
