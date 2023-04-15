use std::collections::HashMap;

pub struct RunResult {
    pub run_index: i32,
    pub status_code: u32,
    pub body: HashMap<String, String>,
    pub successful: bool
}

pub type RunResults = Vec<RunResult>;
