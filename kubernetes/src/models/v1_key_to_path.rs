/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1KeyToPath : Maps a string key to a path within a volume.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1KeyToPath {
  /// The key to project.
  #[serde(rename = "key")]
  key: String,
  /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
  #[serde(rename = "mode")]
  mode: Option<i32>,
  /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
  #[serde(rename = "path")]
  path: String
}

impl V1KeyToPath {
  /// Maps a string key to a path within a volume.
  pub fn new(key: String, path: String) -> V1KeyToPath {
    V1KeyToPath {
      key: key,
      mode: None,
      path: path
    }
  }

  pub fn set_key(&mut self, key: String) {
    self.key = key;
  }

  pub fn with_key(mut self, key: String) -> V1KeyToPath {
    self.key = key;
    self
  }

  pub fn key(&self) -> &String {
    &self.key
  }


  pub fn set_mode(&mut self, mode: i32) {
    self.mode = Some(mode);
  }

  pub fn with_mode(mut self, mode: i32) -> V1KeyToPath {
    self.mode = Some(mode);
    self
  }

  pub fn mode(&self) -> Option<&i32> {
    self.mode.as_ref()
  }

  pub fn reset_mode(&mut self) {
    self.mode = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> V1KeyToPath {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


}



