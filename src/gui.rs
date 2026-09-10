use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::shared::{CameraId, CameraState, ConnId, Deviation, VisionTechnique};

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


// GUI can request with ListEdges
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfEdges {
    pub edges: HashMap<ConnId, String>
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetEdgeName {
    pub edge_id: ConnId,
    pub name: String,
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetEdgeProcessingType {
    pub edge_id: ConnId,
    pub processing_type: VisionTechnique,
}

// Server -> GUI.
// GUI can request this with ListCameras
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfCameras {
    pub cameras: HashMap<String, CameraDetails>, // hashmap has a structure of <(conn_id,camera_id), name>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraDetails {
    pub name: String,
    pub state: CameraState,
    pub warn_below_threshold: f32,
    pub alert_below_threshold: f32,
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetCameraThresholds {
    pub edge_id: ConnId,
    pub camera_id: CameraId,
    pub warn_below_threshold: Deviation,
    pub alert_below_threshold: Deviation,
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetDefaultThreshold {
    pub warn_below_threshold: Deviation,
    pub alert_below_threshold: Deviation,
}

// Server -> GUI
//requested with GetDefaultThresholds
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultThresholds {
    pub warn_below_threshold: Deviation,
    pub alert_below_threshold: Deviation,
}

// Server -> GUI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraDisconnected {
    pub conn_id: ConnId,
    pub camera_id: CameraId,
}

// Server -> GUI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraConnected {
    pub conn_id: ConnId,
    pub camera_id: CameraId,
    pub camera_name: String,
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetNameCamera {
    pub conn_id: ConnId,
    pub camera_id: CameraId,
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
    pub conn_id: ConnId,
    pub camera_id: CameraId,
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
    pub conn_id: ConnId,
    pub camera_id: CameraId,
}

/**

## Live data:
Constant stream of camera data from the edge to the GUI. May later include alerts if we decide GUI is not responsible for detecting such.

* ImageAnalysis (Edge -> Server -> GUI)

*/

// Server -> GUI
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageAnalysis {
    pub conn_id: ConnId,
    pub camera_id: CameraId,
    pub deviation: Deviation
}