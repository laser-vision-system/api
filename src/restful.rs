//! Types used in RESTful communication channels.
#![expect(clippy::doc_markdown, reason = "RESTful is not a code identifier")]

use std::collections::HashSet;
use crate::edge::Settings;
use serde::{Deserialize, Serialize};

/// Post structure for `/edge_logs`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logs {
	pub logs: HashSet<String>,
}

/// Post structure for `/write_default_config`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	pub settings: Settings,
}
