/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1ClusterRole : ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1ClusterRole {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object's metadata.
  #[serde(rename = "metadata")]
  metadata: Option<::models::V1ObjectMeta>,
  /// Rules holds all the PolicyRules for this ClusterRole
  #[serde(rename = "rules")]
  rules: Vec<::models::V1beta1PolicyRule>
}

impl V1beta1ClusterRole {
  /// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
  pub fn new(rules: Vec<::models::V1beta1PolicyRule>) -> V1beta1ClusterRole {
    V1beta1ClusterRole {
      api_version: None,
      kind: None,
      metadata: None,
      rules: rules
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> V1beta1ClusterRole {
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

  pub fn with_kind(mut self, kind: String) -> V1beta1ClusterRole {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::V1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::V1ObjectMeta) -> V1beta1ClusterRole {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::V1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_rules(&mut self, rules: Vec<::models::V1beta1PolicyRule>) {
    self.rules = rules;
  }

  pub fn with_rules(mut self, rules: Vec<::models::V1beta1PolicyRule>) -> V1beta1ClusterRole {
    self.rules = rules;
    self
  }

  pub fn rules(&self) -> &Vec<::models::V1beta1PolicyRule> {
    &self.rules
  }


}



