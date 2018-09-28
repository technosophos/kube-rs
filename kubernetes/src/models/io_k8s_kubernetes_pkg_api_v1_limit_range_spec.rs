/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1LimitRangeSpec : LimitRangeSpec defines a min/max usage limit for resources that match on kind.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1LimitRangeSpec {
  /// Limits is the list of LimitRangeItem objects that are enforced.
  #[serde(rename = "limits")]
  limits: Vec<::models::IoK8sKubernetesPkgApiV1LimitRangeItem>
}

impl IoK8sKubernetesPkgApiV1LimitRangeSpec {
  /// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
  pub fn new(limits: Vec<::models::IoK8sKubernetesPkgApiV1LimitRangeItem>) -> IoK8sKubernetesPkgApiV1LimitRangeSpec {
    IoK8sKubernetesPkgApiV1LimitRangeSpec {
      limits: limits
    }
  }

  pub fn set_limits(&mut self, limits: Vec<::models::IoK8sKubernetesPkgApiV1LimitRangeItem>) {
    self.limits = limits;
  }

  pub fn with_limits(mut self, limits: Vec<::models::IoK8sKubernetesPkgApiV1LimitRangeItem>) -> IoK8sKubernetesPkgApiV1LimitRangeSpec {
    self.limits = limits;
    self
  }

  pub fn limits(&self) -> &Vec<::models::IoK8sKubernetesPkgApiV1LimitRangeItem> {
    &self.limits
  }


}



