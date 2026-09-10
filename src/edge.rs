
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::shared::{CameraId, CameraState, ConnId, Deviation, VisionTechnique};

// Server -> GUI
// Server -> Edge
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorMSG {
    pub code: String,
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialiseConnection {
    pub edge_id: ConnId
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

// Server -> Edge
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetNameCamera {
    pub camera_id: CameraId,
    pub name: String
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterCameras {
    pub cameras: HashMap<CameraId, (String, CameraState)>
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisconnectCamera {
    pub camera_id: CameraId
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BreakCamera {
    pub camera_id: CameraId
}

// Server -> Edge
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProcessingType {
    pub processing_type: VisionTechnique,
}

/**

## Image viewing commands:
Commands related directly to the GUI receiving 'video' from a camera

* StartViewingCamera (GUI -> Server -> Edge)
* StopViewingCamera (GUI -> Server -> Edge)

*/

// Server -> Edge
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartViewingCamera {
    pub camera_id: CameraId,
}

// Server -> Edge: Tells the edge to stop sending the camera feed. Only sent when no more GUIs are watching the given camera feed
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopViewingCamera {
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

// Server -> Edge
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setpoint {
    pub camera_id: CameraId,
}

/**

## Live data:
Constant stream of camera data from the edge to the GUI. May later include alerts if we decide GUI is not responsible for detecting such.

* ImageAnalysis (Edge -> Server -> GUI)

*/

// Edge -> Server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageAnalysis {
    pub camera_id: CameraId,
    pub deviation: Deviation
}




