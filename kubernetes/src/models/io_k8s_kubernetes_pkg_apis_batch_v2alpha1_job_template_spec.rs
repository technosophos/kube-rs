/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisBatchV2alpha1JobTemplateSpec : JobTemplateSpec describes the data a Job should have when created from a template

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisBatchV2alpha1JobTemplateSpec {
  /// Standard object's metadata of the jobs created from this template. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
  /// Specification of the desired behavior of the job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
  #[serde(rename = "spec")]
  spec: Option<::models::IoK8sKubernetesPkgApisBatchV1JobSpec>
}

impl IoK8sKubernetesPkgApisBatchV2alpha1JobTemplateSpec {
  /// JobTemplateSpec describes the data a Job should have when created from a template
  pub fn new() -> IoK8sKubernetesPkgApisBatchV2alpha1JobTemplateSpec {
    IoK8sKubernetesPkgApisBatchV2alpha1JobTemplateSpec {
      metadata: None,
      spec: None
    }
  }

  pub fn set_metadata(&mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sKubernetesPkgApisBatchV2alpha1JobTemplateSpec {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::IoK8sKubernetesPkgApisBatchV1JobSpec) {
    self.spec = Some(spec);
  }

  pub fn with_spec(mut self, spec: ::models::IoK8sKubernetesPkgApisBatchV1JobSpec) -> IoK8sKubernetesPkgApisBatchV2alpha1JobTemplateSpec {
    self.spec = Some(spec);
    self
  }

  pub fn spec(&self) -> Option<&::models::IoK8sKubernetesPkgApisBatchV1JobSpec> {
    self.spec.as_ref()
  }

  pub fn reset_spec(&mut self) {
    self.spec = None;
  }

}



