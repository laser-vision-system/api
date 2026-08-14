use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub camera_id: u32,
    // TBD data type for images
}

// Edge -> Server
// Server <- Edge
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub camera_id: u32,
    pub configuration: String
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageAnalysis {
    pub camera_id: u32,
    pub intensity: f32,
    pub deviation: f32
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterCamera {
    pub cameras: HashMap<u32, String>
}

// Edge -> Server
#[derive(Serialize, Deserialize, Debug)]
pub struct UnregisterCamera {
    pub camera_id: u32
}

// Server -> Edge
#[derive(Serialize, Deserialize, Debug)]
pub struct SetNameCamera {
    pub camera_id: u32,
    pub name: String
}

// Functions for Mutexing pub structs if needed, currently we do not see a use

pub fn image_mutex() -> Mutex<Image> {
    let mutex: Mutex<Image> = Mutex::new(Image {
        camera_id: 0
    });
    return mutex;
}
pub fn configuration_mutex() -> Mutex<Configuration> {
    let mutex: Mutex<Configuration> = Mutex::new( Configuration{
        camera_id: 0,
        configuration: "".to_string()
    });
    return mutex;
}
 pub fn camera_info_mutex() -> Mutex<ImageAnalysis> {
     let mutex: Mutex<ImageAnalysis> = Mutex::new(ImageAnalysis {
         camera_id: 0,
         intensity: 0.0,
         deviation: 0.0
     });
     return mutex
 }
pub fn register_cameras_mutex() -> Mutex<RegisterCamera> {
    let mutex: Mutex<RegisterCamera> = Mutex::new(RegisterCamera {
        cameras: HashMap::new()
    });
    return mutex
}
pub fn unregister_cameras_mutex() -> Mutex<UnregisterCamera> {
    let mutex: Mutex<UnregisterCamera> = Mutex::new(UnregisterCamera {
        camera_id: 0
    });
    return mutex
}
pub fn name_camera_mutex() -> Mutex<SetNameCamera> {
    let mutex: Mutex<SetNameCamera> = Mutex::new(SetNameCamera {
        camera_id: 0,
        name: "".to_string()
    });
    return mutex
}
