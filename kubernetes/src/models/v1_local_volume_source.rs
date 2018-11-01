/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1LocalVolumeSource : Local represents directly-attached storage with node affinity (Beta feature)

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1LocalVolumeSource {
  /// Filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default value is to auto-select a fileystem if unspecified.
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// The full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
  #[serde(rename = "path")]
  path: String
}

impl V1LocalVolumeSource {
  /// Local represents directly-attached storage with node affinity (Beta feature)
  pub fn new(path: String) -> V1LocalVolumeSource {
    V1LocalVolumeSource {
      fs_type: None,
      path: path
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> V1LocalVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> V1LocalVolumeSource {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


}



