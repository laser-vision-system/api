//! Datatypes used for GUI communication.

use crate::shared;
use crate::shared::{
	CameraId, CameraStatus, ConnId, Deviation, ForgetCameraRequest,
	VisionTechnique,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;

/// These are messages sent to the GUI, sent as JSON in the following format:
///
/// {"Name": {"var": ..., ...}}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum GUIRequests {
	ListOfEdges(ListOfEdges),

	ListOfCameras(ListOfCameras),

	DefaultThresholds(DefaultThresholds),

	ImageAnalysis(ImageAnalysis),

	Alert(Alert),

	Configuration(Configuration),

	ForgetCamera(ForgetCamera),

	ProcessingTypes(ProcessingTypes),
}

/// These are all the commands the GUI can send the server - the should be sent in the same format as the MessageToGUI commands arrive.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum ServerRequests {
	ListEdges,

	ListCameras,

	ViewCameraConfig(shared::ViewCameraConfig),

	Configuration(shared::Configuration),

	SetEdgeName(SetEdgeName),

	SetEdgeProcessingType(SetEdgeProcessingType),

	SetCameraThresholds(SetCameraThresholds),

	SetDefaultThreshold(SetDefaultThreshold),

	GetDefaultThresholds,

	SetNameCamera(SetNameCamera),

	StartViewingCamera(StartViewingCamera),

	StopViewingCamera,

	Setpoint(Setpoint),

	ForgetCameraRequest(ForgetCameraRequest),

	ForceRetryCamera(ForceRetryCamera),

	ListProcessingTypes,
}

/// Provides a list of Edges.
///
/// GUI can request with `ListEdges`.
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
/// GUI can request this with `ListCameras`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfCameras {
	// NOTE: Potentially outdated docstring?
	/// hashmap has a structure of <(`conn_id`, `camera_id`), name>.
	pub cameras: HashMap<String, CameraDetails>,
}

/// The information attached to each camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraDetails {
	pub name: String,
	pub state: CameraStatus,
	pub warn_below_threshold: Deviation,
	pub alert_below_threshold: Deviation,
	pub warn_above_threshold: Deviation,
	pub alert_above_threshold: Deviation,
}

/// A request to change the thresholds for a camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetCameraThresholds {
	pub edge_id: ConnId,
	pub camera_id: CameraId,
	pub warn_below_threshold: Deviation,
	pub alert_below_threshold: Deviation,
	pub warn_above_threshold: Deviation,
	pub alert_above_threshold: Deviation,
}

/// A request to change the default threshold.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetDefaultThreshold {
	pub warn_below_threshold: Deviation,
	pub alert_below_threshold: Deviation,
	pub warn_above_threshold: Deviation,
	pub alert_above_threshold: Deviation,
}

/// The current default threshold values.
///
/// Requested with `GetDefaultThresholds`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultThresholds {
	pub warn_below_threshold: Deviation,
	pub alert_below_threshold: Deviation,
	pub warn_above_threshold: Deviation,
	pub alert_above_threshold: Deviation,
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

/// A generic alert message - triggered either by an edge or the server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
	pub conn_id: Option<ConnId>,
	pub camera_id: Option<CameraId>,
	pub alert_code: u32,
	pub alert_type: String,
	pub alert_description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForceRetryCamera {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessingTypes {
	pub processing_types: Vec<String>,
}

impl Default for ProcessingTypes {
	fn default() -> Self {
		let mut processing_types = vec![];
		for technique in VisionTechnique::iter() {
			processing_types.push(format!("{:?}", technique));
		}
		ProcessingTypes { processing_types }
	}
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
