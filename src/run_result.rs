use std::collections::HashMap;

pub struct RequestResult {
    pub run_index: i32,
    pub status_code: u16,
    pub body: HashMap<String, String>,
    pub time_taken: u128,
}

pub type RequestResults = Vec<RequestResult>;

pub struct RunResult {
    id: String,
    request_results: RequestResults,
}

impl RunResult {
    pub fn new(id: String, request_results: RequestResults) -> Self {
        RunResult {
            id,
            request_results,
        }
    }
}
