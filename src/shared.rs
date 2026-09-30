//! Datatypes used in communication with both the Edge and GUI.

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// The type used to uniquely identify a camera.
pub type CameraId = u64;
/// The type used to uniquely identify a connection.
pub type ConnId = u64;
/// The type used to report the current deviation.
pub type Deviation = f32;

/// The current status of the camera device.
///
/// This is a fieldless enum (meaning none of the variants hold data) where the
/// discriminant is the type [`u8`].
///
/// When (de)serialising, it still uses the variant name, not the dicriminant
/// value.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CameraStatus {
	/// This camera device is initialised and ready for use, the stream has been
	/// opened.
	Active,
	/// This camera has indices, but no actual camera device.
	Limbo,
	/// This camera device is not currently connected.
	Disconnected,
	/// This camera can be initialised, but fails to operate correctly.
	Broken,
	/// This device could not be initialised and it should be ignored.
	Invalid,
}

/// Which machine vision approach will be used.
///
/// This selection applies to the entire μ-com device.
#[derive(
	Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, EnumIter,
)]
#[repr(u8)]
#[non_exhaustive]
pub enum VisionTechnique {
	Empty,
	Demonstration,
	Diff,
	Cnn,
}

// == Round trip ==

/// A request to forget about a camera.
///
/// The GUI will initiate the request, which the server will forward to the
/// Edge. The Edge will confirm the request to forget about the camera and
/// return this.
///
// TODO: Is this outdated?
/// When a server becomes disconnected, the client can send one of these
/// commands, and if the response comes back with 'confirm' as true, it means
/// the camera can be removed from the UI (and has been dropped from the edge's
/// memory). Note that confirm should be filled - with any value - when sent
/// from the GUI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetCameraRequest {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
	pub confirm: bool,
}

/// A request to view the camera configuration.
///
/// GUI -> Server. `conn_id` is an edge id.
///
/// Server -> Edge. `conn_id` is the GUI id to be sent back with Configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewCameraConfig {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
}

// TODO: This should be split up.
/// A struct containing a list of camera configuration data.
///
/// Edge -> Server: Sending current config. `conn_id` is a GUI id.
///
/// Edge <- Server: Updating current config. `conn_id` is irrelevant.
///
/// GUI -> Server: Updating current config. `conn_id` is an edge id.
///
/// GUI <- Server: Displaying current config. `conn_id` is an edge id.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
	pub conn_id: ConnId,
	pub camera_id: CameraId,
	pub configuration: String,
}

impl Configuration {
	#[expect(
		clippy::needless_pass_by_value,
		reason = "This is a consuming function"
	)]
	/// Convert a [`ViewCameraConfig`] into a [`Configuration`].
	// TODO: Move to a method on ViewCameraConfig.
	pub fn build_response(request: ViewCameraConfig, response: String) -> Self {
		Self {
			conn_id: request.conn_id,
			camera_id: request.camera_id,
			configuration: response,
		}
	}
}
