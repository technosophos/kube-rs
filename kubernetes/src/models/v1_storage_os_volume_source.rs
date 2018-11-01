/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1StorageOsVolumeSource : Represents a StorageOS persistent volume resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1StorageOsVolumeSource {
  /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
  #[serde(rename = "secretRef")]
  secret_ref: Option<::models::V1LocalObjectReference>,
  /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
  #[serde(rename = "volumeName")]
  volume_name: Option<String>,
  /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to \"default\" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
  #[serde(rename = "volumeNamespace")]
  volume_namespace: Option<String>
}

impl V1StorageOsVolumeSource {
  /// Represents a StorageOS persistent volume resource.
  pub fn new() -> V1StorageOsVolumeSource {
    V1StorageOsVolumeSource {
      fs_type: None,
      read_only: None,
      secret_ref: None,
      volume_name: None,
      volume_namespace: None
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> V1StorageOsVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1StorageOsVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_secret_ref(&mut self, secret_ref: ::models::V1LocalObjectReference) {
    self.secret_ref = Some(secret_ref);
  }

  pub fn with_secret_ref(mut self, secret_ref: ::models::V1LocalObjectReference) -> V1StorageOsVolumeSource {
    self.secret_ref = Some(secret_ref);
    self
  }

  pub fn secret_ref(&self) -> Option<&::models::V1LocalObjectReference> {
    self.secret_ref.as_ref()
  }

  pub fn reset_secret_ref(&mut self) {
    self.secret_ref = None;
  }

  pub fn set_volume_name(&mut self, volume_name: String) {
    self.volume_name = Some(volume_name);
  }

  pub fn with_volume_name(mut self, volume_name: String) -> V1StorageOsVolumeSource {
    self.volume_name = Some(volume_name);
    self
  }

  pub fn volume_name(&self) -> Option<&String> {
    self.volume_name.as_ref()
  }

  pub fn reset_volume_name(&mut self) {
    self.volume_name = None;
  }

  pub fn set_volume_namespace(&mut self, volume_namespace: String) {
    self.volume_namespace = Some(volume_namespace);
  }

  pub fn with_volume_namespace(mut self, volume_namespace: String) -> V1StorageOsVolumeSource {
    self.volume_namespace = Some(volume_namespace);
    self
  }

  pub fn volume_namespace(&self) -> Option<&String> {
    self.volume_namespace.as_ref()
  }

  pub fn reset_volume_namespace(&mut self) {
    self.volume_namespace = None;
  }

}



