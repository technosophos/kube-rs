/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AppsV1beta1ScaleSpec : ScaleSpec describes the attributes of a scale subresource

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppsV1beta1ScaleSpec {
  /// desired number of instances for the scaled object.
  #[serde(rename = "replicas")]
  replicas: Option<i32>
}

impl AppsV1beta1ScaleSpec {
  /// ScaleSpec describes the attributes of a scale subresource
  pub fn new() -> AppsV1beta1ScaleSpec {
    AppsV1beta1ScaleSpec {
      replicas: None
    }
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = Some(replicas);
  }

  pub fn with_replicas(mut self, replicas: i32) -> AppsV1beta1ScaleSpec {
    self.replicas = Some(replicas);
    self
  }

  pub fn replicas(&self) -> Option<&i32> {
    self.replicas.as_ref()
  }

  pub fn reset_replicas(&mut self) {
    self.replicas = None;
  }

}



