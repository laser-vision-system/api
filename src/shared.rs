use serde::{Deserialize, Serialize};

pub type CameraId = u64;
pub type ConnId = u64;
pub type Deviation = f32;

/// The current status of the camera device.
///
/// This is a fieldless enum (meaning none of the variants hold data) where the
/// discriminant is the type [`u8`].
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum CameraStatus {
    /// This camera device has not yet been initialised. This should never be
    /// the status outside of the camera detection logic. If this value has
    /// been sent to the server, it is a major error.
    #[default]
    Uninitialised = 0,
    /// This camera device is initialised and ready for use, the stream has been
    /// opened.
    Active = 1,
    /// This camera device is not currently connected.
    Disconnected = 2,
    /// This camera can be initialised, but fails to operate correctly.
    Broken = 3,
    /// This device could not be initialised.
    Invalid = 4,
}


#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisionTechnique {
    Empty,
    Cnn,
    Diff,
    Demonstration,
}


/**

# Camera list commands:
Commands related to keeping a full list of all cameras.

* \* ListCameras (GUI -> Server)
* ListOfCameras (Server -> GUI)
* CameraDisconnected (Server -> GUI)
* SetNameCamera (GUI -> Server -> Edge)
* RegisterCameras (Edge -> Server)
* UnregisterCamera (Edge -> Server)
* ForgetCamera (round trip). When a server becomes disconnected, the client can send one of these commands,
                             and if the response comes back with 'confirm' as true, it means the camera can be
                             removed from the UI(and has been dropped from the edge's memory). Note that confirm
                             should be filled - with any value - when sent from the GUI.

*/

// Round trip
/// A request to forget about a camera.
///
/// The GUI will initiate the request, which the server will forward to the
/// Edge. The Edge will confirm the request to forget about the camera and
/// return this.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetCameraRequest {
    pub conn_id: ConnId,
    pub camera_id: CameraId,
    pub confirm: bool,
}

/**

## Configuration Commands:
Commands related to displaying or altering the configuration of a camera

* ViewCameraConfig (Edge -> Server -> GUI)
* Configuration (Edge -> Server -> GUI) - Displaying camera config
* Configuration (GUI -> Server -> Edge) - Updating camera config
* Setpoint (GUI -> Server -> Edge) - (Attach no JSON to setpoint all cameras)
* VisionTechnique (round trip) - Change the vision processing implementation per edge client

*/

// GUI -> Server. conn_id is an edge id.
// Server -> Edge. conn_id is the GUI id to be sent back with Configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewCameraConfig {
    pub conn_id: ConnId,
    pub camera_id: CameraId,
}

// Edge -> Server: Sending current config. conn_id is a GUI id.
// Edge <- Server: Updating current config. conn_id is irrelevant.
// GUI -> Server: Updating current config. conn_id is an edge id
// GUI <- Server: Displaying current config. conn_id is an edge id
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub conn_id: ConnId,
    pub camera_id: CameraId,
    pub configuration: String
}

