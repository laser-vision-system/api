//! Datatypes used for Edge communication.

use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use crate::shared::{AlertType, CameraId, CameraStatus, Configuration, ConnId, Deviation, ForgetCameraRequest, ViewCameraConfig, VisionTechnique};

/// The different types of requests the edge will get.
#[expect(clippy::module_name_repetitions, reason = "Not confusing")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(u8)]
#[non_exhaustive]
pub enum EdgeRequests {
	IsCameraCalibrated(IsCameraCalibrated),
	SetNameCamera(SetNameCamera),
	ToggleViewingCamera(ToggleViewingCamera),
	Configuration(Configuration),
	ViewCameraConfig(ViewCameraConfig),
	Setpoint(Setpoint),
	SetProcessingType(SetProcessingType),
	ForceRetryCamera(ForceRetryCamera),
	ForgetCameraRequest(ForgetCameraRequest),
}

/// The different types of requests the edge can send the server
#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum ServerRequests {
	CameraCalibrationStatus(CameraCalibrationStatus),

	CameraCalibrationUpdate(CameraCalibrationUpdate),

	InitialiseConnection(InitialiseConnection),

	RegisterCameras(RegisterCameras),

	UpdateCameraStatus(UpdateCameraStatus),

	ImageAnalysis(ImageAnalysis),

	Configuration(Configuration),

	ForgetCameraRequest(ForgetCameraRequest),

	Alert(Alert),

	SetNameCamera(SetNameCamera),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialiseConnection {
	pub edge_id: ConnId,
}

/// Indicated whether a camera is calibrated.
///
/// Conn_id should be the Conn_id sent with the IsCameraCalibrated command.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraCalibrationStatus {
	pub gui_id: ConnId,
	pub camera_id: CameraId,
	pub calibrated: bool,
}

impl CameraCalibrationStatus{
	#[expect(
		clippy::needless_pass_by_value,
		reason = "This is a consuming function"
	)]
	/// Convert a [`IsCameraCalibrated`] into a [`CameraCalibrationStatus`].
	// TODO: Move to a method on ViewCameraConfig.
	pub fn build_response(request: IsCameraCalibrated, response: bool) -> Self {
		Self {
			gui_id: request.gui_id,
			camera_id: request.camera_id,
			calibrated: response,
		}
	}
}


/// Requests the calibration status of a given camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsCameraCalibrated {
	pub gui_id: ConnId,
	pub camera_id: CameraId,

}

/// Indicates that a camera has changed its calibration status.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraCalibrationUpdate {
	pub camera_id: CameraId,
	pub calibrated: bool,
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

/// A request to change the name of a camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetNameCamera {
	pub camera_id: CameraId,
	pub name: String,
}

/// A request to start or stop sending images from a specific camera.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleViewingCamera {
	pub camera_id: CameraId,
	pub start_viewing: bool,
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

/// A generic alert, which the server will forward to any GUI's
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
	pub camera_id: Option<CameraId>,
	pub alert_type: AlertType,
	pub alert_code: u32,
	pub alert_name: String,
	pub alert_description: String,
}

/// Force the edge to retry connecting to a broken camera
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForceRetryCamera {
	pub camera_id: CameraId,
}
