use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::shared::CameraState;

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

// Server -> GUI.
// GUI can request this with ListCameras
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfCameras {
    pub cameras: HashMap<(u32, u64), (String, CameraState)>, // hashmap has a structure of <(conn_id,camera_id), name>
}

// Server -> GUI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraDisconnected {
    pub conn_id: u32,
    pub camera_id: u64,
}

// Server -> GUI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraConnected {
    pub conn_id: u32,
    pub camera_id: u64,
    pub camera_name: String,
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GUISetNameCamera {
    pub conn_id: u32,
    pub camera_id: u64,
    pub name: String
}

/**

## Image viewing commands:
Commands related directly to the GUI receiving 'video' from a camera

* StartViewingCamera (GUI -> Server -> Edge)
* StopViewingCamera (GUI -> Server -> Edge) (no data required from the gui)

*/

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartViewingCamera {
    pub conn_id: u32,
    pub camera_id: u64,
}

/**

## Configuration Commands:
Commands related to displaying or altering the configuration of a camera

* ViewCameraConfig (Edge -> Server -> GUI)
* Configuration (Edge -> Server -> GUI) - Displaying camera config
* Configuration (GUI -> Server -> Edge) - Updating camera config
* Setpoint (GUI -> Server -> Edge) - (Attach no JSON to setpoint all cameras)

*/

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setpoint {
    pub conn_id: u32,
    pub camera_id: u64,
}

/**

## Live data:
Constant stream of camera data from the edge to the GUI. May later include alerts if we decide GUI is not responsible for detecting such.

* ImageAnalysis (Edge -> Server -> GUI)

*/

// Server -> GUI
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageAnalysis {
    pub conn_id: u32,
    pub camera_id: u64,
    pub intensity: f32,
    pub deviation: f32
}