/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus : PodDisruptionBudgetStatus represents information about the status of a PodDisruptionBudget. Status may trail the actual state of a system.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
  /// current number of healthy pods
  #[serde(rename = "currentHealthy")]
  current_healthy: i32,
  /// minimum desired number of healthy pods
  #[serde(rename = "desiredHealthy")]
  desired_healthy: i32,
  /// DisruptedPods contains information about pods whose eviction was processed by the API server eviction subresource handler but has not yet been observed by the PodDisruptionBudget controller. A pod will be in this map from the time when the API server processed the eviction request to the time when the pod is seen by PDB controller as having been marked for deletion (or after a timeout). The key in the map is the name of the pod and the value is the time when the API server processed the eviction request. If the deletion didn't occur and a pod is still there it will be removed from the list automatically by PodDisruptionBudget controller after some time. If everything goes smooth this map should be empty for the most of the time. Large number of entries in the map may indicate problems with pod deletions.
  #[serde(rename = "disruptedPods")]
  disrupted_pods: ::std::collections::HashMap<String, ::models::IoK8sApimachineryPkgApisMetaV1Time>,
  /// Number of pod disruptions that are currently allowed.
  #[serde(rename = "disruptionsAllowed")]
  disruptions_allowed: i32,
  /// total number of pods counted by this disruption budget
  #[serde(rename = "expectedPods")]
  expected_pods: i32,
  /// Most recent generation observed when updating this PDB status. PodDisruptionsAllowed and other status informatio is valid only if observedGeneration equals to PDB's object generation.
  #[serde(rename = "observedGeneration")]
  observed_generation: Option<i64>
}

impl IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
  /// PodDisruptionBudgetStatus represents information about the status of a PodDisruptionBudget. Status may trail the actual state of a system.
  pub fn new(current_healthy: i32, desired_healthy: i32, disrupted_pods: ::std::collections::HashMap<String, ::models::IoK8sApimachineryPkgApisMetaV1Time>, disruptions_allowed: i32, expected_pods: i32) -> IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
    IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
      current_healthy: current_healthy,
      desired_healthy: desired_healthy,
      disrupted_pods: disrupted_pods,
      disruptions_allowed: disruptions_allowed,
      expected_pods: expected_pods,
      observed_generation: None
    }
  }

  pub fn set_current_healthy(&mut self, current_healthy: i32) {
    self.current_healthy = current_healthy;
  }

  pub fn with_current_healthy(mut self, current_healthy: i32) -> IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
    self.current_healthy = current_healthy;
    self
  }

  pub fn current_healthy(&self) -> &i32 {
    &self.current_healthy
  }


  pub fn set_desired_healthy(&mut self, desired_healthy: i32) {
    self.desired_healthy = desired_healthy;
  }

  pub fn with_desired_healthy(mut self, desired_healthy: i32) -> IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
    self.desired_healthy = desired_healthy;
    self
  }

  pub fn desired_healthy(&self) -> &i32 {
    &self.desired_healthy
  }


  pub fn set_disrupted_pods(&mut self, disrupted_pods: ::std::collections::HashMap<String, ::models::IoK8sApimachineryPkgApisMetaV1Time>) {
    self.disrupted_pods = disrupted_pods;
  }

  pub fn with_disrupted_pods(mut self, disrupted_pods: ::std::collections::HashMap<String, ::models::IoK8sApimachineryPkgApisMetaV1Time>) -> IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
    self.disrupted_pods = disrupted_pods;
    self
  }

  pub fn disrupted_pods(&self) -> &::std::collections::HashMap<String, ::models::IoK8sApimachineryPkgApisMetaV1Time> {
    &self.disrupted_pods
  }


  pub fn set_disruptions_allowed(&mut self, disruptions_allowed: i32) {
    self.disruptions_allowed = disruptions_allowed;
  }

  pub fn with_disruptions_allowed(mut self, disruptions_allowed: i32) -> IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
    self.disruptions_allowed = disruptions_allowed;
    self
  }

  pub fn disruptions_allowed(&self) -> &i32 {
    &self.disruptions_allowed
  }


  pub fn set_expected_pods(&mut self, expected_pods: i32) {
    self.expected_pods = expected_pods;
  }

  pub fn with_expected_pods(mut self, expected_pods: i32) -> IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
    self.expected_pods = expected_pods;
    self
  }

  pub fn expected_pods(&self) -> &i32 {
    &self.expected_pods
  }


  pub fn set_observed_generation(&mut self, observed_generation: i64) {
    self.observed_generation = Some(observed_generation);
  }

  pub fn with_observed_generation(mut self, observed_generation: i64) -> IoK8sKubernetesPkgApisPolicyV1beta1PodDisruptionBudgetStatus {
    self.observed_generation = Some(observed_generation);
    self
  }

  pub fn observed_generation(&self) -> Option<&i64> {
    self.observed_generation.as_ref()
  }

  pub fn reset_observed_generation(&mut self) {
    self.observed_generation = None;
  }

}



