// use std::{collections::HashMap, fmt::Display};
use std::collections::HashMap;
pub type SyscallKey = (u64, u64);

struct SequentialMemoryTable {
    table: HashMap<u64, i64>,
    next: i64,
}

impl SequentialMemoryTable {
    pub fn new() -> SequentialMemoryTable {
        SequentialMemoryTable {
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

impl Default for SequentialMemoryTable {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MetaMemoryTable {
    table: HashMap<u64, SequentialMemoryTable>,
}

impl MetaMemoryTable {
    pub fn new() -> MetaMemoryTable {
        MetaMemoryTable {
            table: HashMap::new(),
        }
    }
}

impl Default for MetaMemoryTable {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MemLookup {
    type Entry;
    fn obtain(&mut self, k: SyscallKey) -> Self::Entry;
}

impl MemLookup for MetaMemoryTable {
    type Entry = i64;
    fn obtain(&mut self, k: SyscallKey) -> i64 {
        let (call, value) = k;
        self.table.entry(call).or_default().obtain(value)
    }
}
