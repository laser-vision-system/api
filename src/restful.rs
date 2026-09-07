use std::collections::HashSet;
use serde::{Deserialize, Serialize};


// Post structure for /edge_logs
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Logs {
    logs: HashSet<String>
}

// Post structure for /write_default_config
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    json: String,
}