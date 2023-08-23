use crate::memtable::MemLookup;
use crate::syscall_table::SyscallTable;
use crate::BoxedError;
use nix::libc::user_regs_struct;
use owo_colors::OwoColorize;

pub struct I64Regs {
    orig_rax: u64,
    rdi: i64,
    rsi: i64,
    rdx: i64,
    rax: i64,
}

pub trait Registers {
    type MemType;
    type RegisterPrinter<'a> = Box<dyn Fn(&Self) + 'a>;
    fn from_regs(
        regs: &user_regs_struct,
        mt: &mut (impl MemLookup<Entry = Self::MemType> + ?Sized),
    ) -> Self;
    fn format(&self, syscall_table: &SyscallTable, color: bool) -> String;
    fn printer<'a>(syscall_table: SyscallTable) -> Result<Self::RegisterPrinter<'a>, BoxedError>;
}

impl Registers for I64Regs {
    type MemType = i64;
    fn from_regs(
        regs: &user_regs_struct,
        mt: &mut (impl MemLookup<Entry = Self::MemType> + ?Sized),
    ) -> I64Regs {
        I64Regs {
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
    fn printer<'a>(syscall_table: SyscallTable) -> Result<Box<dyn Fn(&I64Regs) + 'a>, BoxedError> {
        Ok(Box::new(move |regs: &I64Regs| {
            println!("{}", regs.format(&syscall_table, true));
        }))
    }
}
