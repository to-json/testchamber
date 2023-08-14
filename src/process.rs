use std::{
    io::Error,
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
    pub pre_exec: Option<fn() -> Result<(), Error>>,
}

impl Process {
    pub fn spawn(&mut self) -> &mut Process {
        self.set_pre_exec();
        let child = self.command.as_mut().unwrap().spawn().unwrap();
        let child_pid = Pid::from_raw(child.id() as _);
        self.pid = Some(child_pid);
        self.process = Some(child);
        self
    }

    pub fn set_pre_exec(&mut self) {
        // command.pre_exec is intrinsically 'unsafe'
        let pre_exec = self.pre_exec.unwrap();
        unsafe {
            self.command.as_mut().unwrap().pre_exec(pre_exec);
        };
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
            pre_exec: None,
        }
    }
}
