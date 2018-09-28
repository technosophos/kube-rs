/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1NamespaceStatus : NamespaceStatus is information about the current status of a Namespace.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1NamespaceStatus {
  /// Phase is the current lifecycle phase of the namespace. More info: https://git.k8s.io/community/contributors/design-proposals/namespaces.md#phases
  #[serde(rename = "phase")]
  phase: Option<String>
}

impl IoK8sKubernetesPkgApiV1NamespaceStatus {
  /// NamespaceStatus is information about the current status of a Namespace.
  pub fn new() -> IoK8sKubernetesPkgApiV1NamespaceStatus {
    IoK8sKubernetesPkgApiV1NamespaceStatus {
      phase: None
    }
  }

  pub fn set_phase(&mut self, phase: String) {
    self.phase = Some(phase);
  }

  pub fn with_phase(mut self, phase: String) -> IoK8sKubernetesPkgApiV1NamespaceStatus {
    self.phase = Some(phase);
    self
  }

  pub fn phase(&self) -> Option<&String> {
    self.phase.as_ref()
  }

  pub fn reset_phase(&mut self) {
    self.phase = None;
  }

}



