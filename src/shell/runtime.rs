use std::collections::HashMap;

pub struct Runtime {
    pub cwd: String,
    pub env: HashMap<String, String>,
    pub last_status: i32,
}
impl Runtime {
    pub fn new() -> Self {
        Self {
            cwd: "".into(),
            env: HashMap::new(),
            last_status: 0,
        }
    }
}
