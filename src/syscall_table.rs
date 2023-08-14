use std::{collections::HashMap, fs::File, ops::Index, path::PathBuf};
pub struct SyscallTable {
    call_map: HashMap<u64, String>,
}
impl Index<u64> for SyscallTable {
    type Output = String;
    fn index(&self, idx: u64) -> &Self::Output {
        &&self.call_map[&idx]
    }
}
impl SyscallTable {
    pub fn new(p: String) -> Result<SyscallTable, Box<dyn std::error::Error>> {
        let path = PathBuf::from(p);
        let json: serde_json::Value = serde_json::from_reader(File::open(path)?)?;
        let call_map: HashMap<u64, String> = json["aaData"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| {
                (
                    item[0].as_u64().unwrap(),
                    item[1].as_str().unwrap().to_owned(),
                )
            })
            .collect();
        Ok(SyscallTable { call_map })
    }
}
