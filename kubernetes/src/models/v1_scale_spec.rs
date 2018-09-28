/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ScaleSpec : ScaleSpec describes the attributes of a scale subresource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ScaleSpec {
  /// desired number of instances for the scaled object.
  #[serde(rename = "replicas")]
  replicas: Option<i32>
}

impl V1ScaleSpec {
  /// ScaleSpec describes the attributes of a scale subresource.
  pub fn new() -> V1ScaleSpec {
    V1ScaleSpec {
      replicas: None
    }
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = Some(replicas);
  }

  pub fn with_replicas(mut self, replicas: i32) -> V1ScaleSpec {
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



