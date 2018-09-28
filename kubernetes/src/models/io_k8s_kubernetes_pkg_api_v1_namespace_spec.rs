/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1NamespaceSpec : NamespaceSpec describes the attributes on a Namespace.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1NamespaceSpec {
  /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://git.k8s.io/community/contributors/design-proposals/namespaces.md#finalizers
  #[serde(rename = "finalizers")]
  finalizers: Option<Vec<String>>
}

impl IoK8sKubernetesPkgApiV1NamespaceSpec {
  /// NamespaceSpec describes the attributes on a Namespace.
  pub fn new() -> IoK8sKubernetesPkgApiV1NamespaceSpec {
    IoK8sKubernetesPkgApiV1NamespaceSpec {
      finalizers: None
    }
  }

  pub fn set_finalizers(&mut self, finalizers: Vec<String>) {
    self.finalizers = Some(finalizers);
  }

  pub fn with_finalizers(mut self, finalizers: Vec<String>) -> IoK8sKubernetesPkgApiV1NamespaceSpec {
    self.finalizers = Some(finalizers);
    self
  }

  pub fn finalizers(&self) -> Option<&Vec<String>> {
    self.finalizers.as_ref()
  }

  pub fn reset_finalizers(&mut self) {
    self.finalizers = None;
  }

}



