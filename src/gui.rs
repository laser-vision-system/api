//! Datatypes used for GUI communication.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::{
	CameraId, CameraStatus, ConnId, Deviation, VisionTechnique,
};

/// Provides a list of Edges.
///
/// GUI can request with `ListEdges`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfEdges {
	pub edges: HashMap<String, String>,
}

/// A request to change the name of an Edge.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetEdgeName {
	pub edge_id: ConnId,
	pub name: String,
}

/// A request to change the processing type of an Edge.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetEdgeProcessingType {
	pub edge_id: ConnId,
	pub processing_type: VisionTechnique,
}

/// Provides a list of cameras.
///
/// GUI can request this with `ListCameras`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfCameras {
	// NOTE: Potentially outdated docstring?
	/// hashmap has a structure of <(`conn_id`, `camera_id`), name>
	pub cameras: HashMap<String, CameraDetails>,
}

/// The information attached to each camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraDetails {
	pub name: String,
	pub state: CameraStatus,
	pub warn_below_threshold: f32,
	pub alert_below_threshold: f32,
}

/// A request to change the thresholds for a camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetCameraThresholds {
	pub edge_id: ConnId,
	pub camera_id: CameraId,
	pub warn_below_threshold: Deviation,
	pub alert_below_threshold: Deviation,
}

/// A request to change the default threshold.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetDefaultThreshold {
	pub warn_below_threshold: Deviation,
	pub alert_below_threshold: Deviation,
}

/// The current default threshold values.
///
/// Requested with `GetDefaultThresholds`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultThresholds {
	pub warn_below_threshold: Deviation,
	pub alert_below_threshold: Deviation,
}

/// A notification that a camera has disconnected.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraDisconnected {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
}

/// A notification that a camera has connected.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraConnected {
	pub conn_id: String,
	pub camera_id: String,
	pub camera_name: String,
}

/// A request to set the name of a camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetNameCamera {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
	pub name: String,
}

/// A request to start receiving images from a camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartViewingCamera {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
}

/// A request to calibrate the setpoint of a camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setpoint {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
}

/// A report of the image analysis data from a camera.
///
/// Constant stream of camera data from the edge to the GUI. May later include
/// alerts if we decide GUI is not responsible for detecting such.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageAnalysis {
	pub conn_id: String,
	pub camera_id: CameraId,
	pub deviation: Deviation,
}

// Structs to allow the server to send data to the GUI with IDs in string
// format. Refer to shared for documentation.

/// See [`Configuration`](crate::shared::Configuration).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
	pub conn_id: String,
	pub camera_id: CameraId,
	pub configuration: String,
}

/// See [`ForgetCameraRequest`](crate::shared::ForgetCameraRequest).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetCamera {
	pub conn_id: String,
	pub camera_id: CameraId,
	pub confirm: bool,
}
