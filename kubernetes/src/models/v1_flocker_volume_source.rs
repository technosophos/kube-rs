/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1FlockerVolumeSource : Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1FlockerVolumeSource {
  /// Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
  #[serde(rename = "datasetName")]
  dataset_name: Option<String>,
  /// UUID of the dataset. This is unique identifier of a Flocker dataset
  #[serde(rename = "datasetUUID")]
  dataset_uuid: Option<String>
}

impl V1FlockerVolumeSource {
  /// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.
  pub fn new() -> V1FlockerVolumeSource {
    V1FlockerVolumeSource {
      dataset_name: None,
      dataset_uuid: None
    }
  }

  pub fn set_dataset_name(&mut self, dataset_name: String) {
    self.dataset_name = Some(dataset_name);
  }

  pub fn with_dataset_name(mut self, dataset_name: String) -> V1FlockerVolumeSource {
    self.dataset_name = Some(dataset_name);
    self
  }

  pub fn dataset_name(&self) -> Option<&String> {
    self.dataset_name.as_ref()
  }

  pub fn reset_dataset_name(&mut self) {
    self.dataset_name = None;
  }

  pub fn set_dataset_uuid(&mut self, dataset_uuid: String) {
    self.dataset_uuid = Some(dataset_uuid);
  }

  pub fn with_dataset_uuid(mut self, dataset_uuid: String) -> V1FlockerVolumeSource {
    self.dataset_uuid = Some(dataset_uuid);
    self
  }

  pub fn dataset_uuid(&self) -> Option<&String> {
    self.dataset_uuid.as_ref()
  }

  pub fn reset_dataset_uuid(&mut self) {
    self.dataset_uuid = None;
  }

}



