/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PodTemplateSpec : PodTemplateSpec describes the data a pod should have when created from a template

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1PodTemplateSpec {
  /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::V1ObjectMeta>,
  /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
  #[serde(rename = "spec")]
  spec: Option<::models::V1PodSpec>
}

impl V1PodTemplateSpec {
  /// PodTemplateSpec describes the data a pod should have when created from a template
  pub fn new() -> V1PodTemplateSpec {
    V1PodTemplateSpec {
      metadata: None,
      spec: None
    }
  }

  pub fn set_metadata(&mut self, metadata: ::models::V1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::V1ObjectMeta) -> V1PodTemplateSpec {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::V1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::V1PodSpec) {
    self.spec = Some(spec);
  }

  pub fn with_spec(mut self, spec: ::models::V1PodSpec) -> V1PodTemplateSpec {
    self.spec = Some(spec);
    self
  }

  pub fn spec(&self) -> Option<&::models::V1PodSpec> {
    self.spec.as_ref()
  }

  pub fn reset_spec(&mut self) {
    self.spec = None;
  }

}



