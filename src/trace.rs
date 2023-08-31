use std::io::Error;

use nix::sys::{
    ptrace,
    wait::{waitpid, WaitStatus},
};

use crate::{memtable::MemLookup, normalized_regs::Syscall, process::Process};

pub fn trace<T: Syscall>(
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
                    // todo: holy shit i didn't actually expect this to work yet
                    //       gotta extract this to a macro
                    if regs.orig_rax == 262 {
                        let ptr = regs.rdx;
                        const SIZE: usize = std::mem::size_of::<libc::stat>();
                        let word_size = SIZE.div_ceil(8);
                        let mut target: [u8; SIZE] = [0; SIZE];
                        for i in 0..word_size {
                            // This insane shit reads a number of words from child process
                            // memory sufficient to rehydrate it as the struct it ostensibly
                            // represents according to the syscall docs
                            //
                            // Gotta make it more insane by converting each i64 into
                            // a slice of bytes with to_ne_bytes and then appending
                            // those to target, which should actually be Vec<u8>, and then
                            // truncate target to `size` with Vec::truncate
                            let mut tmp = ptrace::read(child_pid, (ptr.clone() as usize + (i as usize)) as *mut libc::c_void)?.to_ne_bytes();
                            for (j, v) in tmp.iter().enumerate() {
                                let idx = i+j;
                                target[idx] = tmp[j];
                            };

                        }
                        let ret: libc::stat;
                        unsafe {
                            ret = std::mem::transmute(target);
                        };
                    };
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
