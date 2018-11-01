/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ProjectedVolumeSource : Represents a projected volume source

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ProjectedVolumeSource {
  /// Mode bits to use on created files by default. Must be a value between 0 and 0777. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
  #[serde(rename = "defaultMode")]
  default_mode: Option<i32>,
  /// list of volume projections
  #[serde(rename = "sources")]
  sources: Vec<::models::V1VolumeProjection>
}

impl V1ProjectedVolumeSource {
  /// Represents a projected volume source
  pub fn new(sources: Vec<::models::V1VolumeProjection>) -> V1ProjectedVolumeSource {
    V1ProjectedVolumeSource {
      default_mode: None,
      sources: sources
    }
  }

  pub fn set_default_mode(&mut self, default_mode: i32) {
    self.default_mode = Some(default_mode);
  }

  pub fn with_default_mode(mut self, default_mode: i32) -> V1ProjectedVolumeSource {
    self.default_mode = Some(default_mode);
    self
  }

  pub fn default_mode(&self) -> Option<&i32> {
    self.default_mode.as_ref()
  }

  pub fn reset_default_mode(&mut self) {
    self.default_mode = None;
  }

  pub fn set_sources(&mut self, sources: Vec<::models::V1VolumeProjection>) {
    self.sources = sources;
  }

  pub fn with_sources(mut self, sources: Vec<::models::V1VolumeProjection>) -> V1ProjectedVolumeSource {
    self.sources = sources;
    self
  }

  pub fn sources(&self) -> &Vec<::models::V1VolumeProjection> {
    &self.sources
  }


}



