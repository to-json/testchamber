use std::{
    os::unix::process::CommandExt,
    process::{Child, Command},
};

use nix::unistd::Pid;

pub struct Process {
    pub executable: String,
    pub args: Option<Vec<String>>,
    pub command: Option<Command>,
    pub pid: Option<Pid>,
    pub process: Option<Child>,
}

impl Process {
    pub fn spawn(&mut self) -> &mut Process {
        let child = self.command.as_mut().unwrap().spawn().unwrap();
        let child_pid = Pid::from_raw(child.id() as _);
        self.pid = Some(child_pid);
        self.process = Some(child);
        self
    }

    pub fn set_pre_exec(&mut self) -> &mut Process {
        // command.pre_exec is intrinsically 'unsafe'
        unsafe {
            self.command.as_mut().unwrap().pre_exec(|| {
                let mut filter =
                    libseccomp::ScmpFilterContext::new(libseccomp::ScmpAction::Allow).unwrap();
                let _ = filter.add_arch(libseccomp::ScmpArch::X8664);
                let _ = filter.load();
                use nix::sys::ptrace::traceme;
                traceme().map_err(|e| e.into())
            });
        }
        self
    }

    pub fn build_command(&mut self) -> &mut Process {
        let mut cmd = Command::new(&self.executable);
        cmd.args(self.args.as_ref().unwrap());
        self.command = Some(cmd);
        self
    }

    pub fn new(executable: String, args: Option<Vec<String>>) -> Self {
        Self {
            executable,
            args,
            command: None,
            pid: None,
            process: None,
        }
    }
}
