// type SyscallKey = [u64; 2];
use std::collections::HashMap;
pub type SyscallKey = (u64, u64);

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

pub struct MetaMemoryTable {
    table: HashMap<u64, MemoryTable>,
}

impl MetaMemoryTable {
    pub fn new() -> MetaMemoryTable {
        MetaMemoryTable {
            table: HashMap::new(),
        }
    }
}

pub trait MemLookup {
    fn obtain(&mut self, k: SyscallKey) -> i64;
}

impl MemLookup for MetaMemoryTable {
    fn obtain(&mut self, k: SyscallKey) -> i64 {
        let (call, value) = k;
        if self.table.contains_key(&call) {
            self.table.get_mut(&call).unwrap().obtain(value)
        } else {
            self.table.insert(call, MemoryTable::new());
            self.table.get_mut(&call).unwrap().obtain(value)
        }
    }
}
