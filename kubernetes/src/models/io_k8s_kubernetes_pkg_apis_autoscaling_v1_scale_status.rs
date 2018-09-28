/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisAutoscalingV1ScaleStatus : ScaleStatus represents the current status of a scale subresource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAutoscalingV1ScaleStatus {
  /// actual number of observed instances of the scaled object.
  #[serde(rename = "replicas")]
  replicas: i32,
  /// label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
  #[serde(rename = "selector")]
  selector: Option<String>
}

impl IoK8sKubernetesPkgApisAutoscalingV1ScaleStatus {
  /// ScaleStatus represents the current status of a scale subresource.
  pub fn new(replicas: i32) -> IoK8sKubernetesPkgApisAutoscalingV1ScaleStatus {
    IoK8sKubernetesPkgApisAutoscalingV1ScaleStatus {
      replicas: replicas,
      selector: None
    }
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = replicas;
  }

  pub fn with_replicas(mut self, replicas: i32) -> IoK8sKubernetesPkgApisAutoscalingV1ScaleStatus {
    self.replicas = replicas;
    self
  }

  pub fn replicas(&self) -> &i32 {
    &self.replicas
  }


  pub fn set_selector(&mut self, selector: String) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: String) -> IoK8sKubernetesPkgApisAutoscalingV1ScaleStatus {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&String> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

}



