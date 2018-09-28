/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus : DeploymentStatus is the most recently observed status of the Deployment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
  /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
  #[serde(rename = "availableReplicas")]
  available_replicas: Option<i32>,
  /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
  #[serde(rename = "collisionCount")]
  collision_count: Option<i64>,
  /// Represents the latest available observations of a deployment's current state.
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::IoK8sKubernetesPkgApisAppsV1beta1DeploymentCondition>>,
  /// The generation observed by the deployment controller.
  #[serde(rename = "observedGeneration")]
  observed_generation: Option<i64>,
  /// Total number of ready pods targeted by this deployment.
  #[serde(rename = "readyReplicas")]
  ready_replicas: Option<i32>,
  /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
  #[serde(rename = "replicas")]
  replicas: Option<i32>,
  /// Total number of unavailable pods targeted by this deployment.
  #[serde(rename = "unavailableReplicas")]
  unavailable_replicas: Option<i32>,
  /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
  #[serde(rename = "updatedReplicas")]
  updated_replicas: Option<i32>
}

impl IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
  /// DeploymentStatus is the most recently observed status of the Deployment.
  pub fn new() -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
      available_replicas: None,
      collision_count: None,
      conditions: None,
      observed_generation: None,
      ready_replicas: None,
      replicas: None,
      unavailable_replicas: None,
      updated_replicas: None
    }
  }

  pub fn set_available_replicas(&mut self, available_replicas: i32) {
    self.available_replicas = Some(available_replicas);
  }

  pub fn with_available_replicas(mut self, available_replicas: i32) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.available_replicas = Some(available_replicas);
    self
  }

  pub fn available_replicas(&self) -> Option<&i32> {
    self.available_replicas.as_ref()
  }

  pub fn reset_available_replicas(&mut self) {
    self.available_replicas = None;
  }

  pub fn set_collision_count(&mut self, collision_count: i64) {
    self.collision_count = Some(collision_count);
  }

  pub fn with_collision_count(mut self, collision_count: i64) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.collision_count = Some(collision_count);
    self
  }

  pub fn collision_count(&self) -> Option<&i64> {
    self.collision_count.as_ref()
  }

  pub fn reset_collision_count(&mut self) {
    self.collision_count = None;
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::IoK8sKubernetesPkgApisAppsV1beta1DeploymentCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::IoK8sKubernetesPkgApisAppsV1beta1DeploymentCondition>) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApisAppsV1beta1DeploymentCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_observed_generation(&mut self, observed_generation: i64) {
    self.observed_generation = Some(observed_generation);
  }

  pub fn with_observed_generation(mut self, observed_generation: i64) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.observed_generation = Some(observed_generation);
    self
  }

  pub fn observed_generation(&self) -> Option<&i64> {
    self.observed_generation.as_ref()
  }

  pub fn reset_observed_generation(&mut self) {
    self.observed_generation = None;
  }

  pub fn set_ready_replicas(&mut self, ready_replicas: i32) {
    self.ready_replicas = Some(ready_replicas);
  }

  pub fn with_ready_replicas(mut self, ready_replicas: i32) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.ready_replicas = Some(ready_replicas);
    self
  }

  pub fn ready_replicas(&self) -> Option<&i32> {
    self.ready_replicas.as_ref()
  }

  pub fn reset_ready_replicas(&mut self) {
    self.ready_replicas = None;
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = Some(replicas);
  }

  pub fn with_replicas(mut self, replicas: i32) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.replicas = Some(replicas);
    self
  }

  pub fn replicas(&self) -> Option<&i32> {
    self.replicas.as_ref()
  }

  pub fn reset_replicas(&mut self) {
    self.replicas = None;
  }

  pub fn set_unavailable_replicas(&mut self, unavailable_replicas: i32) {
    self.unavailable_replicas = Some(unavailable_replicas);
  }

  pub fn with_unavailable_replicas(mut self, unavailable_replicas: i32) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.unavailable_replicas = Some(unavailable_replicas);
    self
  }

  pub fn unavailable_replicas(&self) -> Option<&i32> {
    self.unavailable_replicas.as_ref()
  }

  pub fn reset_unavailable_replicas(&mut self) {
    self.unavailable_replicas = None;
  }

  pub fn set_updated_replicas(&mut self, updated_replicas: i32) {
    self.updated_replicas = Some(updated_replicas);
  }

  pub fn with_updated_replicas(mut self, updated_replicas: i32) -> IoK8sKubernetesPkgApisAppsV1beta1DeploymentStatus {
    self.updated_replicas = Some(updated_replicas);
    self
  }

  pub fn updated_replicas(&self) -> Option<&i32> {
    self.updated_replicas.as_ref()
  }

  pub fn reset_updated_replicas(&mut self) {
    self.updated_replicas = None;
  }

}



