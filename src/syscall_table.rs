use core::slice::Iter;
use std::{collections::HashMap, fs::File, ops::Index, path::PathBuf};

use serde_json::Value;

use crate::BoxedError;
#[derive(Debug, Clone)]
pub struct SyscallTable {
    call_map: HashMap<u64, String>,
    args_map: HashMap<String, Vec<String>>,
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
        &self.args_map[&idx]
    }
}

impl SyscallTable {
    pub fn new(p: String) -> Result<SyscallTable, BoxedError> {
        let path = PathBuf::from(p);
        let json: serde_json::Value = serde_json::from_reader(File::open(path)?)?;
        let json_iter: Iter<'_, Value> = json["aaData"].as_array().unwrap().iter();
        let _call_map: Option<HashMap<u64, String>> = json_iter.clone()
            .map(|item: &Value| -> Option<(u64, String)> {
                Some((item.get(0)?.as_u64()?, item.get(1)?.as_str()?.to_owned()))
            })
            .collect();
        let call_map = _call_map.ok_or("Unable to parse call map")?;
        // let args: HashMap<String, Vec<String>> = Default::default();
        // fn get_type(arg :Option<&Value>) -> 
        let _args_map: Option<HashMap<String, Vec<String>>> = json_iter
            .map(|item: &Value| -> Option<(String, Vec<String>)> {
                Some((
                    item.get(1)?.as_str()?.to_owned(),
                    vec![
                        item.get(3)?.get("type"),
                        item.get(4)?.get("type"),
                        item.get(5)?.get("type"),
                        item.get(6)?.get("type"),
                        item.get(7)?.get("type"),
                        item.get(8)?.get("type"),
                    ].iter().flatten().map(|x| x.as_str().unwrap().to_owned()).collect()
                ))
            })
            .collect();
        let args_map = _args_map.ok_or("Unable to parse args map")?;
        Ok(SyscallTable {
            call_map,
            args_map,
        })
    }
}
