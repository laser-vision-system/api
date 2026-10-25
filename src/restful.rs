//! Types used in RESTful communication channels.
#![expect(clippy::doc_markdown, reason = "RESTful is not a code identifier")]

use serde::{Deserialize, Serialize};

use crate::edge::Settings;
use crate::shared::ConnId;

/// Post structure for `/edge_logs`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logs {
	pub edge_id: ConnId,
	pub logs: Vec<String>,
}

/// Post structure for `/write_default_config`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	pub settings: Settings,
}
