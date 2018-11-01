/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExtensionsV1beta1DeploymentRollback : DEPRECATED. DeploymentRollback stores the information required to rollback a deployment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsV1beta1DeploymentRollback {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Required: This must match the Name of a deployment.
  #[serde(rename = "name")]
  name: String,
  /// The config of this deployment rollback.
  #[serde(rename = "rollbackTo")]
  rollback_to: ::models::ExtensionsV1beta1RollbackConfig,
  /// The annotations to be updated to a deployment
  #[serde(rename = "updatedAnnotations")]
  updated_annotations: Option<::std::collections::HashMap<String, String>>
}

impl ExtensionsV1beta1DeploymentRollback {
  /// DEPRECATED. DeploymentRollback stores the information required to rollback a deployment.
  pub fn new(name: String, rollback_to: ::models::ExtensionsV1beta1RollbackConfig) -> ExtensionsV1beta1DeploymentRollback {
    ExtensionsV1beta1DeploymentRollback {
      api_version: None,
      kind: None,
      name: name,
      rollback_to: rollback_to,
      updated_annotations: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> ExtensionsV1beta1DeploymentRollback {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> ExtensionsV1beta1DeploymentRollback {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> ExtensionsV1beta1DeploymentRollback {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_rollback_to(&mut self, rollback_to: ::models::ExtensionsV1beta1RollbackConfig) {
    self.rollback_to = rollback_to;
  }

  pub fn with_rollback_to(mut self, rollback_to: ::models::ExtensionsV1beta1RollbackConfig) -> ExtensionsV1beta1DeploymentRollback {
    self.rollback_to = rollback_to;
    self
  }

  pub fn rollback_to(&self) -> &::models::ExtensionsV1beta1RollbackConfig {
    &self.rollback_to
  }


  pub fn set_updated_annotations(&mut self, updated_annotations: ::std::collections::HashMap<String, String>) {
    self.updated_annotations = Some(updated_annotations);
  }

  pub fn with_updated_annotations(mut self, updated_annotations: ::std::collections::HashMap<String, String>) -> ExtensionsV1beta1DeploymentRollback {
    self.updated_annotations = Some(updated_annotations);
    self
  }

  pub fn updated_annotations(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.updated_annotations.as_ref()
  }

  pub fn reset_updated_annotations(&mut self) {
    self.updated_annotations = None;
  }

}



