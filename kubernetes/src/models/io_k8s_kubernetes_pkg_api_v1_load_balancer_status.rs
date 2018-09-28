/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1LoadBalancerStatus : LoadBalancerStatus represents the status of a load-balancer.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1LoadBalancerStatus {
  /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
  #[serde(rename = "ingress")]
  ingress: Option<Vec<::models::IoK8sKubernetesPkgApiV1LoadBalancerIngress>>
}

impl IoK8sKubernetesPkgApiV1LoadBalancerStatus {
  /// LoadBalancerStatus represents the status of a load-balancer.
  pub fn new() -> IoK8sKubernetesPkgApiV1LoadBalancerStatus {
    IoK8sKubernetesPkgApiV1LoadBalancerStatus {
      ingress: None
    }
  }

  pub fn set_ingress(&mut self, ingress: Vec<::models::IoK8sKubernetesPkgApiV1LoadBalancerIngress>) {
    self.ingress = Some(ingress);
  }

  pub fn with_ingress(mut self, ingress: Vec<::models::IoK8sKubernetesPkgApiV1LoadBalancerIngress>) -> IoK8sKubernetesPkgApiV1LoadBalancerStatus {
    self.ingress = Some(ingress);
    self
  }

  pub fn ingress(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApiV1LoadBalancerIngress>> {
    self.ingress.as_ref()
  }

  pub fn reset_ingress(&mut self) {
    self.ingress = None;
  }

}



