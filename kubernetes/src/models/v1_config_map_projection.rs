/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ConfigMapProjection : Adapts a ConfigMap into a projected volume.  The contents of the target ConfigMap's Data field will be presented in a projected volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. Note that this is identical to a configmap volume source without the default mode.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ConfigMapProjection {
  /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
  #[serde(rename = "items")]
  items: Option<Vec<::models::V1KeyToPath>>,
  /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specify whether the ConfigMap or it's keys must be defined
  #[serde(rename = "optional")]
  optional: Option<bool>
}

impl V1ConfigMapProjection {
  /// Adapts a ConfigMap into a projected volume.  The contents of the target ConfigMap's Data field will be presented in a projected volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. Note that this is identical to a configmap volume source without the default mode.
  pub fn new() -> V1ConfigMapProjection {
    V1ConfigMapProjection {
      items: None,
      name: None,
      optional: None
    }
  }

  pub fn set_items(&mut self, items: Vec<::models::V1KeyToPath>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::V1KeyToPath>) -> V1ConfigMapProjection {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::V1KeyToPath>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> V1ConfigMapProjection {
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

  pub fn with_optional(mut self, optional: bool) -> V1ConfigMapProjection {
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



