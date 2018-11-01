/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta2DaemonSetStatus : DaemonSetStatus represents the current status of a daemon set.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta2DaemonSetStatus {
  /// Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
  #[serde(rename = "collisionCount")]
  collision_count: Option<i32>,
  /// Represents the latest available observations of a DaemonSet's current state.
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::V1beta2DaemonSetCondition>>,
  /// The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
  #[serde(rename = "currentNumberScheduled")]
  current_number_scheduled: i32,
  /// The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
  #[serde(rename = "desiredNumberScheduled")]
  desired_number_scheduled: i32,
  /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds)
  #[serde(rename = "numberAvailable")]
  number_available: Option<i32>,
  /// The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
  #[serde(rename = "numberMisscheduled")]
  number_misscheduled: i32,
  /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and ready.
  #[serde(rename = "numberReady")]
  number_ready: i32,
  /// The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)
  #[serde(rename = "numberUnavailable")]
  number_unavailable: Option<i32>,
  /// The most recent generation observed by the daemon set controller.
  #[serde(rename = "observedGeneration")]
  observed_generation: Option<i64>,
  /// The total number of nodes that are running updated daemon pod
  #[serde(rename = "updatedNumberScheduled")]
  updated_number_scheduled: Option<i32>
}

impl V1beta2DaemonSetStatus {
  /// DaemonSetStatus represents the current status of a daemon set.
  pub fn new(current_number_scheduled: i32, desired_number_scheduled: i32, number_misscheduled: i32, number_ready: i32) -> V1beta2DaemonSetStatus {
    V1beta2DaemonSetStatus {
      collision_count: None,
      conditions: None,
      current_number_scheduled: current_number_scheduled,
      desired_number_scheduled: desired_number_scheduled,
      number_available: None,
      number_misscheduled: number_misscheduled,
      number_ready: number_ready,
      number_unavailable: None,
      observed_generation: None,
      updated_number_scheduled: None
    }
  }

  pub fn set_collision_count(&mut self, collision_count: i32) {
    self.collision_count = Some(collision_count);
  }

  pub fn with_collision_count(mut self, collision_count: i32) -> V1beta2DaemonSetStatus {
    self.collision_count = Some(collision_count);
    self
  }

  pub fn collision_count(&self) -> Option<&i32> {
    self.collision_count.as_ref()
  }

  pub fn reset_collision_count(&mut self) {
    self.collision_count = None;
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::V1beta2DaemonSetCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::V1beta2DaemonSetCondition>) -> V1beta2DaemonSetStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::V1beta2DaemonSetCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_current_number_scheduled(&mut self, current_number_scheduled: i32) {
    self.current_number_scheduled = current_number_scheduled;
  }

  pub fn with_current_number_scheduled(mut self, current_number_scheduled: i32) -> V1beta2DaemonSetStatus {
    self.current_number_scheduled = current_number_scheduled;
    self
  }

  pub fn current_number_scheduled(&self) -> &i32 {
    &self.current_number_scheduled
  }


  pub fn set_desired_number_scheduled(&mut self, desired_number_scheduled: i32) {
    self.desired_number_scheduled = desired_number_scheduled;
  }

  pub fn with_desired_number_scheduled(mut self, desired_number_scheduled: i32) -> V1beta2DaemonSetStatus {
    self.desired_number_scheduled = desired_number_scheduled;
    self
  }

  pub fn desired_number_scheduled(&self) -> &i32 {
    &self.desired_number_scheduled
  }


  pub fn set_number_available(&mut self, number_available: i32) {
    self.number_available = Some(number_available);
  }

  pub fn with_number_available(mut self, number_available: i32) -> V1beta2DaemonSetStatus {
    self.number_available = Some(number_available);
    self
  }

  pub fn number_available(&self) -> Option<&i32> {
    self.number_available.as_ref()
  }

  pub fn reset_number_available(&mut self) {
    self.number_available = None;
  }

  pub fn set_number_misscheduled(&mut self, number_misscheduled: i32) {
    self.number_misscheduled = number_misscheduled;
  }

  pub fn with_number_misscheduled(mut self, number_misscheduled: i32) -> V1beta2DaemonSetStatus {
    self.number_misscheduled = number_misscheduled;
    self
  }

  pub fn number_misscheduled(&self) -> &i32 {
    &self.number_misscheduled
  }


  pub fn set_number_ready(&mut self, number_ready: i32) {
    self.number_ready = number_ready;
  }

  pub fn with_number_ready(mut self, number_ready: i32) -> V1beta2DaemonSetStatus {
    self.number_ready = number_ready;
    self
  }

  pub fn number_ready(&self) -> &i32 {
    &self.number_ready
  }


  pub fn set_number_unavailable(&mut self, number_unavailable: i32) {
    self.number_unavailable = Some(number_unavailable);
  }

  pub fn with_number_unavailable(mut self, number_unavailable: i32) -> V1beta2DaemonSetStatus {
    self.number_unavailable = Some(number_unavailable);
    self
  }

  pub fn number_unavailable(&self) -> Option<&i32> {
    self.number_unavailable.as_ref()
  }

  pub fn reset_number_unavailable(&mut self) {
    self.number_unavailable = None;
  }

  pub fn set_observed_generation(&mut self, observed_generation: i64) {
    self.observed_generation = Some(observed_generation);
  }

  pub fn with_observed_generation(mut self, observed_generation: i64) -> V1beta2DaemonSetStatus {
    self.observed_generation = Some(observed_generation);
    self
  }

  pub fn observed_generation(&self) -> Option<&i64> {
    self.observed_generation.as_ref()
  }

  pub fn reset_observed_generation(&mut self) {
    self.observed_generation = None;
  }

  pub fn set_updated_number_scheduled(&mut self, updated_number_scheduled: i32) {
    self.updated_number_scheduled = Some(updated_number_scheduled);
  }

  pub fn with_updated_number_scheduled(mut self, updated_number_scheduled: i32) -> V1beta2DaemonSetStatus {
    self.updated_number_scheduled = Some(updated_number_scheduled);
    self
  }

  pub fn updated_number_scheduled(&self) -> Option<&i32> {
    self.updated_number_scheduled.as_ref()
  }

  pub fn reset_updated_number_scheduled(&mut self) {
    self.updated_number_scheduled = None;
  }

}



