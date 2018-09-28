/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus : ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus {
  /// currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.  It will only be present if `targetAverageValue` was set in the corresponding metric specification.
  #[serde(rename = "currentAverageUtilization")]
  current_average_utilization: Option<i32>,
  /// currentAverageValue is the current value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the \"pods\" metric source type. It will always be set, regardless of the corresponding metric specification.
  #[serde(rename = "currentAverageValue")]
  current_average_value: ::models::IoK8sApimachineryPkgApiResourceQuantity,
  /// name is the name of the resource in question.
  #[serde(rename = "name")]
  name: String
}

impl IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus {
  /// ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.
  pub fn new(current_average_value: ::models::IoK8sApimachineryPkgApiResourceQuantity, name: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus {
    IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus {
      current_average_utilization: None,
      current_average_value: current_average_value,
      name: name
    }
  }

  pub fn set_current_average_utilization(&mut self, current_average_utilization: i32) {
    self.current_average_utilization = Some(current_average_utilization);
  }

  pub fn with_current_average_utilization(mut self, current_average_utilization: i32) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus {
    self.current_average_utilization = Some(current_average_utilization);
    self
  }

  pub fn current_average_utilization(&self) -> Option<&i32> {
    self.current_average_utilization.as_ref()
  }

  pub fn reset_current_average_utilization(&mut self) {
    self.current_average_utilization = None;
  }

  pub fn set_current_average_value(&mut self, current_average_value: ::models::IoK8sApimachineryPkgApiResourceQuantity) {
    self.current_average_value = current_average_value;
  }

  pub fn with_current_average_value(mut self, current_average_value: ::models::IoK8sApimachineryPkgApiResourceQuantity) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus {
    self.current_average_value = current_average_value;
    self
  }

  pub fn current_average_value(&self) -> &::models::IoK8sApimachineryPkgApiResourceQuantity {
    &self.current_average_value
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sKubernetesPkgApisAutoscalingV2alpha1ResourceMetricStatus {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



