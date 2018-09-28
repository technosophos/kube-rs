/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec : MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
  /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
  #[serde(rename = "object")]
  object: Option<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource>,
  /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
  #[serde(rename = "pods")]
  pods: Option<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricSource>,
  /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.
  #[serde(rename = "resource")]
  resource: Option<::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricSource>,
  /// type is the type of metric source.  It should match one of the fields below.
  #[serde(rename = "type")]
  _type: String
}

impl IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
  /// MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).
  pub fn new(_type: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
    IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
      object: None,
      pods: None,
      resource: None,
      _type: _type
    }
  }

  pub fn set_object(&mut self, object: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource) {
    self.object = Some(object);
  }

  pub fn with_object(mut self, object: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
    self.object = Some(object);
    self
  }

  pub fn object(&self) -> Option<&::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ObjectMetricSource> {
    self.object.as_ref()
  }

  pub fn reset_object(&mut self) {
    self.object = None;
  }

  pub fn set_pods(&mut self, pods: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricSource) {
    self.pods = Some(pods);
  }

  pub fn with_pods(mut self, pods: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricSource) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
    self.pods = Some(pods);
    self
  }

  pub fn pods(&self) -> Option<&::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1PodsMetricSource> {
    self.pods.as_ref()
  }

  pub fn reset_pods(&mut self) {
    self.pods = None;
  }

  pub fn set_resource(&mut self, resource: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricSource) {
    self.resource = Some(resource);
  }

  pub fn with_resource(mut self, resource: ::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricSource) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
    self.resource = Some(resource);
    self
  }

  pub fn resource(&self) -> Option<&::models::IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricSource> {
    self.resource.as_ref()
  }

  pub fn reset_resource(&mut self) {
    self.resource = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1MetricSpec {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



