use std::io::Error;

use nix::sys::{
    ptrace,
    wait::{waitpid, WaitStatus},
};

use crate::{memtable::MemLookup, normalized_regs::Registers, process::Process};

pub fn trace<T: Registers>(
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
