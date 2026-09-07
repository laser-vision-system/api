use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CameraState {
    Active,
    Disconnected,
    Broken
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetCamera {
    pub conn_id: u32,
    pub camera_id: u64,
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
    pub conn_id: u32,
    pub camera_id: u64,
}

// Edge -> Server: Sending current config. conn_id is a GUI id.
// Edge <- Server: Updating current config. conn_id is irrelevant.
// GUI -> Server: Updating current config. conn_id is an edge id
// GUI <- Server: Displaying current config. conn_id is an edge id
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub conn_id: u32,
    pub camera_id: u64,
    pub configuration: String
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisionTechnique {
	Empty,
	Cnn,
	Diff,
	Demonstration,
}
