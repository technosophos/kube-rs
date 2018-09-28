/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sApimachineryPkgApisMetaV1OwnerReference : OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApimachineryPkgApisMetaV1OwnerReference {
  /// API version of the referent.
  #[serde(rename = "apiVersion")]
  api_version: String,
  /// If true, AND if the owner has the \"foregroundDeletion\" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs \"delete\" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
  #[serde(rename = "blockOwnerDeletion")]
  block_owner_deletion: Option<bool>,
  /// If true, this reference points to the managing controller.
  #[serde(rename = "controller")]
  controller: Option<bool>,
  /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: String,
  /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
  #[serde(rename = "name")]
  name: String,
  /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
  #[serde(rename = "uid")]
  uid: String
}

impl IoK8sApimachineryPkgApisMetaV1OwnerReference {
  /// OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field.
  pub fn new(api_version: String, kind: String, name: String, uid: String) -> IoK8sApimachineryPkgApisMetaV1OwnerReference {
    IoK8sApimachineryPkgApisMetaV1OwnerReference {
      api_version: api_version,
      block_owner_deletion: None,
      controller: None,
      kind: kind,
      name: name,
      uid: uid
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = api_version;
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sApimachineryPkgApisMetaV1OwnerReference {
    self.api_version = api_version;
    self
  }

  pub fn api_version(&self) -> &String {
    &self.api_version
  }


  pub fn set_block_owner_deletion(&mut self, block_owner_deletion: bool) {
    self.block_owner_deletion = Some(block_owner_deletion);
  }

  pub fn with_block_owner_deletion(mut self, block_owner_deletion: bool) -> IoK8sApimachineryPkgApisMetaV1OwnerReference {
    self.block_owner_deletion = Some(block_owner_deletion);
    self
  }

  pub fn block_owner_deletion(&self) -> Option<&bool> {
    self.block_owner_deletion.as_ref()
  }

  pub fn reset_block_owner_deletion(&mut self) {
    self.block_owner_deletion = None;
  }

  pub fn set_controller(&mut self, controller: bool) {
    self.controller = Some(controller);
  }

  pub fn with_controller(mut self, controller: bool) -> IoK8sApimachineryPkgApisMetaV1OwnerReference {
    self.controller = Some(controller);
    self
  }

  pub fn controller(&self) -> Option<&bool> {
    self.controller.as_ref()
  }

  pub fn reset_controller(&mut self) {
    self.controller = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = kind;
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sApimachineryPkgApisMetaV1OwnerReference {
    self.kind = kind;
    self
  }

  pub fn kind(&self) -> &String {
    &self.kind
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sApimachineryPkgApisMetaV1OwnerReference {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_uid(&mut self, uid: String) {
    self.uid = uid;
  }

  pub fn with_uid(mut self, uid: String) -> IoK8sApimachineryPkgApisMetaV1OwnerReference {
    self.uid = uid;
    self
  }

  pub fn uid(&self) -> &String {
    &self.uid
  }


}



