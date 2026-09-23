//! Datatypes used for Edge communication.

use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

use crate::shared::{
	CameraId, CameraStatus, Configuration, ConnId, Deviation, ViewCameraConfig,
	VisionTechnique,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorMSG {
	pub code: String,
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialiseConnection {
	pub edge_id: ConnId,
}

/// Information about a newly instantiated device.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewCameraInfo {
	pub name: String,
	pub status: CameraStatus,
}

impl NewCameraInfo {
	pub fn new(name: String, status: CameraStatus) -> Self {
		Self { name, status }
	}
}

impl From<(String, CameraStatus)> for NewCameraInfo {
	fn from(value: (String, CameraStatus)) -> Self {
		NewCameraInfo {
			name: value.0,
			status: value.1,
		}
	}
}

/// System-level default settings for the Edge.
///
/// This will be provided by the initial connection to the server.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Settings {
	pub websocket_poll_interval: Duration,
	pub update_delay: Duration,
	pub camera_err_decrement: u16,
	pub camera_err_increment: u16,
	pub camera_err_threshold: u16,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			websocket_poll_interval: Duration::from_millis(10),
			update_delay: Duration::from_secs(1),
			camera_err_decrement: 1,
			camera_err_increment: 2,
			camera_err_threshold: 5,
		}
	}
}

/// Report to the server an update of the camera devices.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterCameras {
	pub cameras: HashMap<CameraId, NewCameraInfo>,
}

/// Report to the server an update of a camera status.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCameraStatus {
	pub camera_id: CameraId,
	pub status: CameraStatus,
}

/// Report to the server the result of the image analysis.
///
/// Constant stream of camera data from the edge to the GUI. May later include
/// alerts if we decide GUI is not responsible for detecting such.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageAnalysis {
	pub camera_id: CameraId,
	pub deviation: Deviation,
}

// == Requests ==

/// The different types of requests the edge will get.
///
/// Not intended to be sent to the edge itself.
#[expect(clippy::module_name_repetitions, reason = "Not confusing")]
#[derive(Clone, Debug)]
#[repr(u8)]
#[non_exhaustive]
pub enum EdgeRequests {
	SetNameCamera(SetNameCamera),
	StartViewingCamera(StartViewingCamera),
	StopViewingCamera(StopViewingCamera),
	Configuration(Configuration),
	ViewCameraConfig(ViewCameraConfig),
	Setpoint(Setpoint),
	SetProcessingType(SetProcessingType),
}

/// A request to change the name of a camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetNameCamera {
	pub camera_id: CameraId,
	pub name: String,
}

/// A request to start sending images.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartViewingCamera {
	pub camera_id: CameraId,
}

/// Tells the edge to stop sending the camera feed.
///
/// Only sent when no more GUIs are watching the given camera feed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopViewingCamera {
	pub camera_id: CameraId,
}

/// A request to calibrate the setpoint on the next measurement.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setpoint {
	pub camera_id: CameraId,
}

/// A request to swap the processing type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProcessingType {
	pub processing_type: VisionTechnique,
}
