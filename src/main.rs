use std::{
    collections::HashMap,
    fs::File,
    path::PathBuf, ops::Index,
};

use clap::Parser;
use nix::{
    libc::user_regs_struct,
    sys::{
        ptrace,
        wait::{waitpid, WaitStatus},
    },
};
use owo_colors::OwoColorize;

mod process;
use process::Process;

mod memtable;
use memtable::{MemLookup, MetaMemoryTable};

struct NormalizedRegs {
    orig_rax: u64,
    rdi: i64,
    rsi: i64,
    rdx: i64,
    rax: i64,
}

impl NormalizedRegs {
    fn from_regs(regs: &user_regs_struct, mt: &mut dyn MemLookup) -> NormalizedRegs {
        NormalizedRegs {
            orig_rax: regs.orig_rax,
            rdi: mt.obtain((regs.orig_rax, regs.rdi)),
            rsi: mt.obtain((regs.orig_rax, regs.rsi)),
            rdx: mt.obtain((regs.orig_rax, regs.rdx)),
            rax: mt.obtain((regs.orig_rax, regs.rax)),
        }
    }
    fn format(&self, syscall_table: &SyscallTable, color: bool) -> String {
        if color {
            format!(
                "{}({:x}, {:x}, {:x}, ...) = {:x}",
                syscall_table[self.orig_rax].green(),
                self.rdi.blue(),
                self.rsi.blue(),
                self.rdx.blue(),
                self.rax.yellow(),
            )
        } else {
            format!(
                "{}({:x}, {:x}, {:x}, ...) = {:x}",
                syscall_table[self.orig_rax], self.rdi, self.rsi, self.rdx, self.rax,
            )
        }
    }
}

fn print_normalized_syscall(syscall_table: &SyscallTable, regs: NormalizedRegs) {
    println!("{}", regs.format(syscall_table, true));
}

struct SyscallTable {
    map: HashMap<u64, String>
}
impl Index<u64> for SyscallTable{
    type Output = String;
    fn index(&self, idx: u64) -> &Self::Output {
        &&self.map[&idx]
    }

}
impl SyscallTable {
    fn new(path: PathBuf) -> Result<SyscallTable, Box<dyn std::error::Error>> {
        let json: serde_json::Value = serde_json::from_reader(File::open(path)?)?;
        let call_map: HashMap<u64, String> = json["aaData"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| {
                (
                    item[0].as_u64().unwrap(),
                    item[1].as_str().unwrap().to_owned(),
                )
            })
            .collect();
        Ok(SyscallTable{map: call_map})
    }
}


fn trace(
    process: &mut Process,
    memory_table: &mut dyn MemLookup,
    syscall_table: &SyscallTable,
) -> Result<(), Box<dyn std::error::Error>> {
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
                    let normalized_regs = NormalizedRegs::from_regs(&regs, memory_table);
                    print_normalized_syscall(&syscall_table, normalized_regs);
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
    let syscall_table = SyscallTable::new(PathBuf::from(cli.syscall_table_path))?;
    let mut memory_table = MetaMemoryTable::new();
    let (executable, args) = cli.command.split_first().unwrap();
    let mut cmd = Process::new(executable.to_string(), Some(args.into()));
    cmd.build_command().set_pre_exec().spawn();
    trace(&mut cmd, &mut memory_table, &syscall_table)
}
