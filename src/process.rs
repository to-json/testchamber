use nix::unistd::Pid;
use std::process::Command;
use std::{io::Error, os::unix::process::CommandExt, process::Child};

pub struct Process {
    pub executable: String,
    pub args: Option<Vec<String>>,
    pub command: Option<Command>,
    pub pid: Option<Pid>,
    pub process: Option<Child>,
    pub pre_exec: Option<fn() -> Result<(), Error>>,
}

impl Process {
    /// Fork the command, setting the Process `child` to a `std::process::Child`
    /// and the Process `pid` to the pid of the aforementioned `Child`
    pub fn spawn(&mut self) -> &mut Process {
        let child = self.command.as_mut().unwrap().spawn().unwrap();
        let child_pid = Pid::from_raw(child.id() as _);
        self.pid = Some(child_pid);
        self.process = Some(child);
        self
    }

    /// Run the `pre_exec` closure provided to the Process, returning a Result
    /// containing the Command, an error produced by the `pre_exec` closure,
    /// or an error indicating no `pre_exec` closure was provided
    pub fn set_pre_exec(&mut self) -> Result<&mut std::process::Command, Error> {
        // command.pre_exec is intrinsically 'unsafe'
        match self.pre_exec {
            Some(_) => unsafe {
                return Ok(self
                    .command
                    .as_mut()
                    .unwrap()
                    .pre_exec(self.pre_exec.unwrap()));
            },
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "Unable to read pre_exec function",
            )),
        }
    }

    /// Combines the command name provided, and the arguments, if any, to produce a
    /// `std::process::Command` to be used by subsequent methods, and assigns that
    /// to the `command` on the `Process`
    pub fn build_command(&mut self) -> &mut Process {
        let mut cmd = Command::new(&self.executable);
        cmd.args(self.args.as_ref().unwrap());
        self.command = Some(cmd);
        self
    }

    /// Create a new Process from a program name and, optionally, it's arguments
    ///
    /// * `executable`: a String of the command to be run
    /// * `args`: an Option(al) Vec of Strings containing arguments for the command
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
