use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use embedded_can::Id;
use tokio::sync::Mutex;

pub struct StartupInfo(BTreeMap<String, HashMap<String, String>>);

impl StartupInfo {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self(BTreeMap::new())))
    }

    pub fn add_task_info(&mut self, task_name: &str, info: HashMap<String, String>) {
        self.0.insert(task_name.to_string(), info);
    }

    pub fn output(&self) -> String {
        let mut output = String::new();

        output.push_str("CANopen config:\n");

        for (task_name, info) in &self.0 {
            output.push_str(&format!("{}:\n", task_name));
            for (key, value) in info {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
        }

        output
    }
}

pub fn id_to_string(id: Id) -> String {
    match id {
        Id::Standard(id) => format!("0x{:03x}", id.as_raw()),
        Id::Extended(id) => format!("0x{:08x}", id.as_raw()),
    }
}
