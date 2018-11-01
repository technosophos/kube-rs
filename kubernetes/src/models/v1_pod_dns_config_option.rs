/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PodDnsConfigOption : PodDNSConfigOption defines DNS resolver options of a pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1PodDnsConfigOption {
  /// Required.
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "value")]
  value: Option<String>
}

impl V1PodDnsConfigOption {
  /// PodDNSConfigOption defines DNS resolver options of a pod.
  pub fn new() -> V1PodDnsConfigOption {
    V1PodDnsConfigOption {
      name: None,
      value: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> V1PodDnsConfigOption {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_value(&mut self, value: String) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: String) -> V1PodDnsConfigOption {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&String> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

}



