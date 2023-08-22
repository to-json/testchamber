use core::slice::Iter;
use std::{collections::HashMap, fs::File, ops::Index, path::PathBuf};

use serde_json::Value;
pub struct SyscallTable {
    call_map: HashMap<u64, String>,
    _args: HashMap<String, Vec<String>>,
}
impl Index<u64> for SyscallTable {
    type Output = String;
    fn index(&self, idx: u64) -> &Self::Output {
        &self.call_map[&idx]
    }
}

impl Index<String> for SyscallTable {
    type Output = Vec<String>;
    fn index(&self, idx: String) -> &Self::Output {
        &self._args[&idx]
    }
}

impl SyscallTable {
    pub fn new(p: String) -> Result<SyscallTable, Box<dyn std::error::Error>> {
        let path = PathBuf::from(p);
        let json: serde_json::Value = serde_json::from_reader(File::open(path)?)?;
        let json_iter: Iter<'_, Value> = json["aaData"].as_array().unwrap().iter();
        let _call_map: Option<HashMap<u64, String>> = json_iter
            .map(|item: &Value| -> Option<(u64, String)> {
                Some((item.get(0)?.as_u64()?, item.get(1)?.as_str()?.to_owned()))
            })
            .collect();
        let call_map = _call_map.ok_or("Unable to parse call map")?;
        let args: HashMap<String, Vec<String>> = Default::default();
        Ok(SyscallTable {
            call_map,
            _args: args,
        })
    }
}
