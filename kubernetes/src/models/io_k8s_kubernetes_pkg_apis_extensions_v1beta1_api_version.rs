/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisExtensionsV1beta1ApiVersion : An APIVersion represents a single concrete version of an object model.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisExtensionsV1beta1ApiVersion {
  /// Name of this version (e.g. 'v1').
  #[serde(rename = "name")]
  name: Option<String>
}

impl IoK8sKubernetesPkgApisExtensionsV1beta1ApiVersion {
  /// An APIVersion represents a single concrete version of an object model.
  pub fn new() -> IoK8sKubernetesPkgApisExtensionsV1beta1ApiVersion {
    IoK8sKubernetesPkgApisExtensionsV1beta1ApiVersion {
      name: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1ApiVersion {
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



