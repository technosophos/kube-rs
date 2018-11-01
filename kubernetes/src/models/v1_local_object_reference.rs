/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1LocalObjectReference : LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1LocalObjectReference {
  /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
  #[serde(rename = "name")]
  name: Option<String>
}

impl V1LocalObjectReference {
  /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
  pub fn new() -> V1LocalObjectReference {
    V1LocalObjectReference {
      name: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> V1LocalObjectReference {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



