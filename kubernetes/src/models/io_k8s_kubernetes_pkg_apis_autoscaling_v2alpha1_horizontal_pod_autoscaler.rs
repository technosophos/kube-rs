/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler : HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler, which automatically manages the replica count of any resource implementing the scale subresource based on the metrics specified.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// metadata is the standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
  /// spec is the specification for the behaviour of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
  #[serde(rename = "spec")]
  spec: Option<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerSpec>,
  /// status is the current information about the autoscaler.
  #[serde(rename = "status")]
  status: Option<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerStatus>
}

impl IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
  /// HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler, which automatically manages the replica count of any resource implementing the scale subresource based on the metrics specified.
  pub fn new() -> IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
    IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
      api_version: None,
      kind: None,
      metadata: None,
      spec: None,
      status: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerSpec) {
    self.spec = Some(spec);
  }

  pub fn with_spec(mut self, spec: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerSpec) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
    self.spec = Some(spec);
    self
  }

  pub fn spec(&self) -> Option<&::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerSpec> {
    self.spec.as_ref()
  }

  pub fn reset_spec(&mut self) {
    self.spec = None;
  }

  pub fn set_status(&mut self, status: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerStatus) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscaler {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1HorizontalPodAutoscalerStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



