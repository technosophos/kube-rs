/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec : PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
  /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
  #[serde(rename = "accessModes")]
  access_modes: Option<Vec<String>>,
  /// Resources represents the minimum resources the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
  #[serde(rename = "resources")]
  resources: Option<::models::IoK8sKubernetesPkgApiV1ResourceRequirements>,
  /// A label query over volumes to consider for binding.
  #[serde(rename = "selector")]
  selector: Option<::models::IoK8sApimachineryPkgApisMetaV1LabelSelector>,
  /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
  #[serde(rename = "storageClassName")]
  storage_class_name: Option<String>,
  /// VolumeName is the binding reference to the PersistentVolume backing this claim.
  #[serde(rename = "volumeName")]
  volume_name: Option<String>
}

impl IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
  /// PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes
  pub fn new() -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
    IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
      access_modes: None,
      resources: None,
      selector: None,
      storage_class_name: None,
      volume_name: None
    }
  }

  pub fn set_access_modes(&mut self, access_modes: Vec<String>) {
    self.access_modes = Some(access_modes);
  }

  pub fn with_access_modes(mut self, access_modes: Vec<String>) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
    self.access_modes = Some(access_modes);
    self
  }

  pub fn access_modes(&self) -> Option<&Vec<String>> {
    self.access_modes.as_ref()
  }

  pub fn reset_access_modes(&mut self) {
    self.access_modes = None;
  }

  pub fn set_resources(&mut self, resources: ::models::IoK8sKubernetesPkgApiV1ResourceRequirements) {
    self.resources = Some(resources);
  }

  pub fn with_resources(mut self, resources: ::models::IoK8sKubernetesPkgApiV1ResourceRequirements) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
    self.resources = Some(resources);
    self
  }

  pub fn resources(&self) -> Option<&::models::IoK8sKubernetesPkgApiV1ResourceRequirements> {
    self.resources.as_ref()
  }

  pub fn reset_resources(&mut self) {
    self.resources = None;
  }

  pub fn set_selector(&mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

  pub fn set_storage_class_name(&mut self, storage_class_name: String) {
    self.storage_class_name = Some(storage_class_name);
  }

  pub fn with_storage_class_name(mut self, storage_class_name: String) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
    self.storage_class_name = Some(storage_class_name);
    self
  }

  pub fn storage_class_name(&self) -> Option<&String> {
    self.storage_class_name.as_ref()
  }

  pub fn reset_storage_class_name(&mut self) {
    self.storage_class_name = None;
  }

  pub fn set_volume_name(&mut self, volume_name: String) {
    self.volume_name = Some(volume_name);
  }

  pub fn with_volume_name(mut self, volume_name: String) -> IoK8sKubernetesPkgApiV1PersistentVolumeClaimSpec {
    self.volume_name = Some(volume_name);
    self
  }

  pub fn volume_name(&self) -> Option<&String> {
    self.volume_name.as_ref()
  }

  pub fn reset_volume_name(&mut self) {
    self.volume_name = None;
  }

}



