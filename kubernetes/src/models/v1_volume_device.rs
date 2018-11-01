/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1VolumeDevice : volumeDevice describes a mapping of a raw block device within a container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1VolumeDevice {
  /// devicePath is the path inside of the container that the device will be mapped to.
  #[serde(rename = "devicePath")]
  device_path: String,
  /// name must match the name of a persistentVolumeClaim in the pod
  #[serde(rename = "name")]
  name: String
}

impl V1VolumeDevice {
  /// volumeDevice describes a mapping of a raw block device within a container.
  pub fn new(device_path: String, name: String) -> V1VolumeDevice {
    V1VolumeDevice {
      device_path: device_path,
      name: name
    }
  }

  pub fn set_device_path(&mut self, device_path: String) {
    self.device_path = device_path;
  }

  pub fn with_device_path(mut self, device_path: String) -> V1VolumeDevice {
    self.device_path = device_path;
    self
  }

  pub fn device_path(&self) -> &String {
    &self.device_path
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> V1VolumeDevice {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



