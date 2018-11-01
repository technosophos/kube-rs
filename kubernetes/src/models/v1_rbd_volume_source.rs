/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1RbdVolumeSource : Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1RbdVolumeSource {
  /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// The rados image name. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
  #[serde(rename = "image")]
  image: String,
  /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
  #[serde(rename = "keyring")]
  keyring: Option<String>,
  /// A collection of Ceph monitors. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
  #[serde(rename = "monitors")]
  monitors: Vec<String>,
  /// The rados pool name. Default is rbd. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
  #[serde(rename = "pool")]
  pool: Option<String>,
  /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  /// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
  #[serde(rename = "secretRef")]
  secret_ref: Option<::models::V1LocalObjectReference>,
  /// The rados user name. Default is admin. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
  #[serde(rename = "user")]
  user: Option<String>
}

impl V1RbdVolumeSource {
  /// Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.
  pub fn new(image: String, monitors: Vec<String>) -> V1RbdVolumeSource {
    V1RbdVolumeSource {
      fs_type: None,
      image: image,
      keyring: None,
      monitors: monitors,
      pool: None,
      read_only: None,
      secret_ref: None,
      user: None
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> V1RbdVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_image(&mut self, image: String) {
    self.image = image;
  }

  pub fn with_image(mut self, image: String) -> V1RbdVolumeSource {
    self.image = image;
    self
  }

  pub fn image(&self) -> &String {
    &self.image
  }


  pub fn set_keyring(&mut self, keyring: String) {
    self.keyring = Some(keyring);
  }

  pub fn with_keyring(mut self, keyring: String) -> V1RbdVolumeSource {
    self.keyring = Some(keyring);
    self
  }

  pub fn keyring(&self) -> Option<&String> {
    self.keyring.as_ref()
  }

  pub fn reset_keyring(&mut self) {
    self.keyring = None;
  }

  pub fn set_monitors(&mut self, monitors: Vec<String>) {
    self.monitors = monitors;
  }

  pub fn with_monitors(mut self, monitors: Vec<String>) -> V1RbdVolumeSource {
    self.monitors = monitors;
    self
  }

  pub fn monitors(&self) -> &Vec<String> {
    &self.monitors
  }


  pub fn set_pool(&mut self, pool: String) {
    self.pool = Some(pool);
  }

  pub fn with_pool(mut self, pool: String) -> V1RbdVolumeSource {
    self.pool = Some(pool);
    self
  }

  pub fn pool(&self) -> Option<&String> {
    self.pool.as_ref()
  }

  pub fn reset_pool(&mut self) {
    self.pool = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1RbdVolumeSource {
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

  pub fn with_secret_ref(mut self, secret_ref: ::models::V1LocalObjectReference) -> V1RbdVolumeSource {
    self.secret_ref = Some(secret_ref);
    self
  }

  pub fn secret_ref(&self) -> Option<&::models::V1LocalObjectReference> {
    self.secret_ref.as_ref()
  }

  pub fn reset_secret_ref(&mut self) {
    self.secret_ref = None;
  }

  pub fn set_user(&mut self, user: String) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: String) -> V1RbdVolumeSource {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&String> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



