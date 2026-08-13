use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Image {
    pub camera_id: i32,

}
fn image_mutex() -> Mutex<Image> {
    let mutex: Mutex<Image> = Mutex::new(Image {
        camera_id: 0
    });
    return mutex;
}
#[derive(Serialize, Deserialize)]
struct Configuration {
    pub camera_id: i32,
    pub configuration: String
}
fn configuration_mutex() -> Mutex<Configuration> {
    let mutex: Mutex<Configuration> = Mutex::new( Configuration{
        camera_id: 0,
        configuration: "".to_string()
    });
    return mutex;
}
#[derive(Serialize, Deserialize)]
struct CameraInfo {
    pub intensity: f32,
    pub deviation: f32
}
 fn camera_info_mutex() -> Mutex<CameraInfo> {
     let mutex: Mutex<CameraInfo> = Mutex::new(CameraInfo {
         intensity: 0.0,
         deviation: 0.0
     });
     return mutex
 }
#[derive(Serialize, Deserialize)]
struct RegisterCameras {
    cameras: HashMap<i32, String>
}
fn register_cameras_mutex() -> Mutex<RegisterCameras> {
    let mutex: Mutex<RegisterCameras> = Mutex::new(RegisterCameras {
        cameras: HashMap::new()
    });
    return mutex
}
#[derive(Serialize, Deserialize)]
struct UnregisterCameras {
    camera_id: i32
}
fn unregister_cameras_mutex() -> Mutex<UnregisterCameras> {
    let mutex: Mutex<UnregisterCameras> = Mutex::new(UnregisterCameras {
        camera_id: 0
    });
    return mutex

}
#[derive(Serialize, Deserialize)]
struct NameCamera {
    camera_id: i32,
    name: String
}
fn name_camera_mutex() -> Mutex<NameCamera> {
    let mutex: Mutex<NameCamera> = Mutex::new(NameCamera {
        camera_id: 0,
        name: "".to_string()
    });
    return mutex
}
