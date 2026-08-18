use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CameraState {
    Active,
    Disconnected,
    Broken
}

// Server -> GUI
// Server -> Edge
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMSG {
    pub code: String,
    pub message: String,
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

// Server -> GUI.
// GUI can request this with ListCameras
#[derive(Serialize, Deserialize, Debug)]
pub struct ListOfCameras {
    pub cameras: HashMap<(u32, u64), (String, CameraState)>, // hashmap has a structure of <(conn_id,camera_id), name>
}

// Server -> GUI.
#[derive(Serialize, Deserialize, Debug)]
pub struct CameraDisconnected {
    pub conn_id: u32,
    pub camera_id: u64,
}

// Server -> GUI.
#[derive(Serialize, Deserialize, Debug)]
pub struct CameraConnected {
    pub conn_id: u32,
    pub camera_id: u64,
    pub camera_name: String,
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct GUISetNameCamera {
    pub conn_id: u32,
    pub camera_id: u64,
    pub name: String
}

// Server -> Edge
#[derive(Serialize, Deserialize, Debug)]
pub struct SetNameCamera {
    pub camera_id: u64,
    pub name: String
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterCameras {
    pub cameras: HashMap<u64, (String, CameraState)>
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct DisconnectCamera {
    pub camera_id: u64
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct BreakCamera {
    pub camera_id: u64
}

// Round trip
#[derive(Serialize, Deserialize, Debug)]
pub struct ForgetCamera {
    pub conn_id: u32,
    pub camera_id: u64,
    pub confirm: bool,
}

/**

## Image viewing commands:
Commands related directly to the GUI receiving 'video' from a camera

* Image(tbd)(Edge -> Server -> GUI)
* StartViewingCamera (GUI -> Server -> Edge)
* StopViewingCamera (GUI -> Server -> Edge)

*/

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub camera_id: u64,
    // TBD data type for images
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct GUIStartViewingCamera {
    pub conn_id: u32,
    pub camera_id: u64,
}

// Server -> Edge
#[derive(Serialize, Deserialize, Debug)]
pub struct StartViewingCamera {
    pub camera_id: u64,
}

// Server -> Edge: Tells the edge to stop sending the camera feed. Only sent when no more GUIs are watching the given camera feed
#[derive(Serialize, Deserialize, Debug)]
pub struct StopViewingCamera {
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


// GUI -> Server. conn_id is an edge id.
// Server -> Edge. conn_id is the GUI id to be sent back with Configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct ViewCameraConfig {
    pub conn_id: u32,
    pub camera_id: u64,
}

// Edge -> Server: Sending current config. conn_id is a GUI id.
// Edge <- Server: Updating current config. conn_id is irrelevant.
// GUI -> Server: Updating current config. conn_id is an edge id
// GUI <- Server: Displaying current config. conn_id is an edge id
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub conn_id: u32,
    pub camera_id: u64,
    pub configuration: String
}

// GUI -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct GUISetpoint {
    pub conn_id: u32,
    pub camera_id: u64,
}

// Server -> Edge
#[derive(Serialize, Deserialize, Debug)]
pub struct Setpoint {
    pub camera_id: u64,
}


/**

## Live data:
Constant stream of camera data from the edge to the GUI. May later include alerts if we decide GUI is not responsible for detecting such.

* ImageAnalysis (Edge -> Server -> GUI)

*/

// Edge -> Server.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageAnalysis {
    pub camera_id: u64,
    pub intensity: f32,
    pub deviation: f32
}

// Server -> GUI
#[derive(Serialize, Deserialize, Debug)]
pub struct GUIImageAnalysis {
    pub conn_id: u32,
    pub camera_id: u64,
    pub intensity: f32,
    pub deviation: f32
}



// Functions for Mutexing pub structs if needed, currently we do not see a use

pub fn image_mutex() -> Mutex<Image> {
    let mutex: Mutex<Image> = Mutex::new(Image {
        camera_id: 0
    });
    mutex
}
pub fn configuration_mutex() -> Mutex<Configuration> {
    let mutex: Mutex<Configuration> = Mutex::new( Configuration{
        conn_id: 0,
        camera_id: 0,
        configuration: "".to_string()
    });
    mutex
}
pub fn camera_info_mutex() -> Mutex<ImageAnalysis> {
    let mutex: Mutex<ImageAnalysis> = Mutex::new(ImageAnalysis {
        camera_id: 0,
        intensity: 0.0,
        deviation: 0.0
    });
    mutex
}
pub fn register_cameras_mutex() -> Mutex<RegisterCameras> {
    let mutex: Mutex<RegisterCameras> = Mutex::new(RegisterCameras {
        cameras: HashMap::new()
    });
    mutex
}
pub fn unregister_cameras_mutex() -> Mutex<DisconnectCamera> {
    let mutex: Mutex<DisconnectCamera> = Mutex::new(DisconnectCamera {
        camera_id: 0
    });
    mutex
}
pub fn name_camera_mutex() -> Mutex<SetNameCamera> {
    let mutex: Mutex<SetNameCamera> = Mutex::new(SetNameCamera {
        camera_id: 0,
        name: "".to_string()
    });
    mutex
}
