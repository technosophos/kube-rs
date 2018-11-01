/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1SecretEnvSource : SecretEnvSource selects a Secret to populate the environment variables with.  The contents of the target Secret's Data field will represent the key-value pairs as environment variables.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1SecretEnvSource {
  /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specify whether the Secret must be defined
  #[serde(rename = "optional")]
  optional: Option<bool>
}

impl V1SecretEnvSource {
  /// SecretEnvSource selects a Secret to populate the environment variables with.  The contents of the target Secret's Data field will represent the key-value pairs as environment variables.
  pub fn new() -> V1SecretEnvSource {
    V1SecretEnvSource {
      name: None,
      optional: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> V1SecretEnvSource {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_optional(&mut self, optional: bool) {
    self.optional = Some(optional);
  }

  pub fn with_optional(mut self, optional: bool) -> V1SecretEnvSource {
    self.optional = Some(optional);
    self
  }

  pub fn optional(&self) -> Option<&bool> {
    self.optional.as_ref()
  }

  pub fn reset_optional(&mut self) {
    self.optional = None;
  }

}



