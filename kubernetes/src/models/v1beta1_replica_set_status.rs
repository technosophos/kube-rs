/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1ReplicaSetStatus : ReplicaSetStatus represents the current status of a ReplicaSet.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1ReplicaSetStatus {
  /// The number of available replicas (ready for at least minReadySeconds) for this replica set.
  #[serde(rename = "availableReplicas")]
  available_replicas: Option<i32>,
  /// Represents the latest available observations of a replica set's current state.
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::V1beta1ReplicaSetCondition>>,
  /// The number of pods that have labels matching the labels of the pod template of the replicaset.
  #[serde(rename = "fullyLabeledReplicas")]
  fully_labeled_replicas: Option<i32>,
  /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
  #[serde(rename = "observedGeneration")]
  observed_generation: Option<i64>,
  /// The number of ready replicas for this replica set.
  #[serde(rename = "readyReplicas")]
  ready_replicas: Option<i32>,
  /// Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
  #[serde(rename = "replicas")]
  replicas: i32
}

impl V1beta1ReplicaSetStatus {
  /// ReplicaSetStatus represents the current status of a ReplicaSet.
  pub fn new(replicas: i32) -> V1beta1ReplicaSetStatus {
    V1beta1ReplicaSetStatus {
      available_replicas: None,
      conditions: None,
      fully_labeled_replicas: None,
      observed_generation: None,
      ready_replicas: None,
      replicas: replicas
    }
  }

  pub fn set_available_replicas(&mut self, available_replicas: i32) {
    self.available_replicas = Some(available_replicas);
  }

  pub fn with_available_replicas(mut self, available_replicas: i32) -> V1beta1ReplicaSetStatus {
    self.available_replicas = Some(available_replicas);
    self
  }

  pub fn available_replicas(&self) -> Option<&i32> {
    self.available_replicas.as_ref()
  }

  pub fn reset_available_replicas(&mut self) {
    self.available_replicas = None;
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::V1beta1ReplicaSetCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::V1beta1ReplicaSetCondition>) -> V1beta1ReplicaSetStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::V1beta1ReplicaSetCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_fully_labeled_replicas(&mut self, fully_labeled_replicas: i32) {
    self.fully_labeled_replicas = Some(fully_labeled_replicas);
  }

  pub fn with_fully_labeled_replicas(mut self, fully_labeled_replicas: i32) -> V1beta1ReplicaSetStatus {
    self.fully_labeled_replicas = Some(fully_labeled_replicas);
    self
  }

  pub fn fully_labeled_replicas(&self) -> Option<&i32> {
    self.fully_labeled_replicas.as_ref()
  }

  pub fn reset_fully_labeled_replicas(&mut self) {
    self.fully_labeled_replicas = None;
  }

  pub fn set_observed_generation(&mut self, observed_generation: i64) {
    self.observed_generation = Some(observed_generation);
  }

  pub fn with_observed_generation(mut self, observed_generation: i64) -> V1beta1ReplicaSetStatus {
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

  pub fn with_ready_replicas(mut self, ready_replicas: i32) -> V1beta1ReplicaSetStatus {
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
    self.replicas = replicas;
  }

  pub fn with_replicas(mut self, replicas: i32) -> V1beta1ReplicaSetStatus {
    self.replicas = replicas;
    self
  }

  pub fn replicas(&self) -> &i32 {
    &self.replicas
  }


}



