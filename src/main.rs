use clap::Parser;
use nix::errno::Errno;
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
use normalized_regs::NormalizedRegs;

fn trace(
    process: &mut Process,
    memory_table: &mut dyn MemLookup,
    printer: Box<dyn Fn(&NormalizedRegs)>,
) -> Result<(), Errno> {
    // every syscall has an entrance and exit point. in order to only log the
    // syscall once, we toggle a var every loop
    let pre_exec = || -> Result<(), Errno>{
                let mut filter =
                    libseccomp::ScmpFilterContext::new(libseccomp::ScmpAction::Allow).unwrap();
                let _ = filter.add_arch(libseccomp::ScmpArch::X8664);
                let _ = filter.load();
                use nix::sys::ptrace::traceme;
                traceme()
    };
// 
    process.pre_exec = Some(Box::new(pre_exec));

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
    let print_syscall = NormalizedRegs::syscall_printer(syscall_table)?;
    let mut memory_table = MetaMemoryTable::new();
    let (executable, args) = cli.command.split_first().unwrap();
    let mut cmd = Process::new(executable.to_string(), Some(args.into()));
    cmd.build_command().set_pre_exec().spawn();
    trace(&mut cmd, &mut memory_table, print_syscall).map_err(|e| Errno::into(e))
}
