/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ReplicationControllerList : ReplicationControllerList is a collection of replication controllers.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ReplicationControllerList {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// List of replication controllers. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller
  #[serde(rename = "items")]
  items: Vec<::models::V1ReplicationController>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "metadata")]
  metadata: Option<::models::V1ListMeta>
}

impl V1ReplicationControllerList {
  /// ReplicationControllerList is a collection of replication controllers.
  pub fn new(items: Vec<::models::V1ReplicationController>) -> V1ReplicationControllerList {
    V1ReplicationControllerList {
      api_version: None,
      items: items,
      kind: None,
      metadata: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> V1ReplicationControllerList {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::V1ReplicationController>) {
    self.items = items;
  }

  pub fn with_items(mut self, items: Vec<::models::V1ReplicationController>) -> V1ReplicationControllerList {
    self.items = items;
    self
  }

  pub fn items(&self) -> &Vec<::models::V1ReplicationController> {
    &self.items
  }


  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> V1ReplicationControllerList {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::V1ListMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::V1ListMeta) -> V1ReplicationControllerList {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::V1ListMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

}



