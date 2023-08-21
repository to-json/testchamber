use crate::{MemLookup, SyscallTable};
use nix::libc::user_regs_struct;
use owo_colors::OwoColorize;

pub struct NormalizedRegs {
    orig_rax: u64,
    rdi: i64,
    rsi: i64,
    rdx: i64,
    rax: i64,
}

impl NormalizedRegs {
    pub fn from_regs(regs: &user_regs_struct, mt: &mut dyn MemLookup<Entry=i64>) -> NormalizedRegs {
        NormalizedRegs {
            orig_rax: regs.orig_rax,
            rdi: mt.obtain((regs.orig_rax, regs.rdi)),
            rsi: mt.obtain((regs.orig_rax, regs.rsi)),
            rdx: mt.obtain((regs.orig_rax, regs.rdx)),
            rax: mt.obtain((regs.orig_rax, regs.rax)),
        }
    }
    pub fn format(&self, syscall_table: &SyscallTable, color: bool) -> String {
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
    pub fn syscall_printer<'a>(
        syscall_table: SyscallTable,
    ) -> Result<Box<dyn Fn(&NormalizedRegs) + 'a>, Box<dyn std::error::Error>> {
        Ok(Box::new(move |regs: &NormalizedRegs| {
            println!("{}", regs.format(&syscall_table, true));
        }))
    }
}
