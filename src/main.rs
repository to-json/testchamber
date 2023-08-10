#![feature(inherent_associated_types)]
use std::{
    collections::HashMap,
    fs::File,
    path::PathBuf,
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

type SyscallKey = [u64; 2];

struct MemoryTable {
    table: HashMap<u64, i64>,
    next: i64,
}
impl MemoryTable {
    pub fn new() -> MemoryTable {
        MemoryTable {
            next: 0,
            table: HashMap::new(),
        }
    }

    fn append(&mut self, k: u64) -> i64 {
        let idx = self.next;
        self.table.insert(k, idx);
        self.next += 1;
        idx
    }

    // obtain as "lookup-or-append"
    fn obtain(&mut self, k: u64) -> i64 {
        if self.table.contains_key(&k) {
            self.table[&k]
        } else {
            self.append(k)
        }
    }
}

struct MetaMemoryTable {
    table: HashMap<u64, MemoryTable>,
}

impl MetaMemoryTable {
    fn new() -> MetaMemoryTable {
        MetaMemoryTable {
            table: HashMap::new(),
        }
    }
}

trait MemLookup {
    type MemKey;
    fn obtain(&mut self, k: Self::MemKey) -> i64;
}

impl MemLookup for MetaMemoryTable {
    type MemKey = SyscallKey;
    fn obtain(&mut self, k: SyscallKey) -> i64 {
        let [call, value] = k;
        if self.table.contains_key(&call) {
            self.table.get_mut(&call).unwrap().obtain(value)
        } else {
            self.table.insert(call, MemoryTable::new());
            self.table.get_mut(&call).unwrap().obtain(value)
        }
    }
}

struct NormalizedRegs {
    orig_rax: u64,
    rdi: i64,
    rsi: i64,
    rdx: i64,
    rax: i64,
}

impl NormalizedRegs {
    fn from_regs(regs: &user_regs_struct, mt: &mut dyn MemLookup<MemKey = SyscallKey>) -> NormalizedRegs {
        NormalizedRegs {
            orig_rax: regs.orig_rax,
            rdi: mt.obtain([regs.orig_rax, regs.rdi]),
            rsi: mt.obtain([regs.orig_rax, regs.rsi]),
            rdx: mt.obtain([regs.orig_rax, regs.rdx]),
            rax: mt.obtain([regs.orig_rax, regs.rax]),
        }
    }
    fn format(&self, syscall_table: &SyscallTable, color: bool) -> String {
        if color {
            format!(
                "{}({:x}, {:x}, {:x}, ...) = {:x}",
                syscall_table[&self.orig_rax].green(),
                self.rdi.blue(),
                self.rsi.blue(),
                self.rdx.blue(),
                self.rax.yellow(),
            )
        } else {
            format!(
                "{}({:x}, {:x}, {:x}, ...) = {:x}",
                syscall_table[&self.orig_rax], self.rdi, self.rsi, self.rdx, self.rax,
            )
        }
    }
}

// print_normalized_syscall(&syscall_table, normalized_regs);
fn print_normalized_syscall(syscall_table: &SyscallTable, regs: NormalizedRegs) {
    println!("{}", regs.format(syscall_table, true));
}

type SyscallTable = HashMap<u64, String>;
fn load_syscall_table(path: PathBuf) -> Result<HashMap<u64, String>, Box<dyn std::error::Error>> {
    let json: serde_json::Value = serde_json::from_reader(File::open(path)?)?;
    let syscall_table: HashMap<u64, String> = json["aaData"]
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
    Ok(syscall_table)
}


fn trace(
    process: &mut Process,
    memory_table: &mut dyn MemLookup<MemKey = SyscallKey>,
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
    let syscall_table = load_syscall_table(PathBuf::from(cli.syscall_table_path))?;
    let mut memory_table = MetaMemoryTable::new();
    let (executable, args) = cli.command.split_first().unwrap();
    let mut cmd = Process::new(executable.to_string(), Some(args.into()));
    cmd.build_command().set_pre_exec().spawn();
    trace(&mut cmd, &mut memory_table, &syscall_table)
}
