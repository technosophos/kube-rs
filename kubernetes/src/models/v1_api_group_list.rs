/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ApiGroupList : APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ApiGroupList {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// groups is a list of APIGroup.
  #[serde(rename = "groups")]
  groups: Vec<::models::V1ApiGroup>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>
}

impl V1ApiGroupList {
  /// APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.
  pub fn new(groups: Vec<::models::V1ApiGroup>) -> V1ApiGroupList {
    V1ApiGroupList {
      api_version: None,
      groups: groups,
      kind: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> V1ApiGroupList {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_groups(&mut self, groups: Vec<::models::V1ApiGroup>) {
    self.groups = groups;
  }

  pub fn with_groups(mut self, groups: Vec<::models::V1ApiGroup>) -> V1ApiGroupList {
    self.groups = groups;
    self
  }

  pub fn groups(&self) -> &Vec<::models::V1ApiGroup> {
    &self.groups
  }


  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> V1ApiGroupList {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

}



