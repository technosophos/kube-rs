/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V2alpha1HorizontalPodAutoscalerStatus : HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2alpha1HorizontalPodAutoscalerStatus {
  /// conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
  #[serde(rename = "conditions")]
  conditions: Vec<::models::V2alpha1HorizontalPodAutoscalerCondition>,
  /// currentMetrics is the last read state of the metrics used by this autoscaler.
  #[serde(rename = "currentMetrics")]
  current_metrics: Vec<::models::V2alpha1MetricStatus>,
  /// currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
  #[serde(rename = "currentReplicas")]
  current_replicas: i32,
  /// desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
  #[serde(rename = "desiredReplicas")]
  desired_replicas: i32,
  /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
  #[serde(rename = "lastScaleTime")]
  last_scale_time: Option<String>,
  /// observedGeneration is the most recent generation observed by this autoscaler.
  #[serde(rename = "observedGeneration")]
  observed_generation: Option<i64>
}

impl V2alpha1HorizontalPodAutoscalerStatus {
  /// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
  pub fn new(conditions: Vec<::models::V2alpha1HorizontalPodAutoscalerCondition>, current_metrics: Vec<::models::V2alpha1MetricStatus>, current_replicas: i32, desired_replicas: i32) -> V2alpha1HorizontalPodAutoscalerStatus {
    V2alpha1HorizontalPodAutoscalerStatus {
      conditions: conditions,
      current_metrics: current_metrics,
      current_replicas: current_replicas,
      desired_replicas: desired_replicas,
      last_scale_time: None,
      observed_generation: None
    }
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::V2alpha1HorizontalPodAutoscalerCondition>) {
    self.conditions = conditions;
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::V2alpha1HorizontalPodAutoscalerCondition>) -> V2alpha1HorizontalPodAutoscalerStatus {
    self.conditions = conditions;
    self
  }

  pub fn conditions(&self) -> &Vec<::models::V2alpha1HorizontalPodAutoscalerCondition> {
    &self.conditions
  }


  pub fn set_current_metrics(&mut self, current_metrics: Vec<::models::V2alpha1MetricStatus>) {
    self.current_metrics = current_metrics;
  }

  pub fn with_current_metrics(mut self, current_metrics: Vec<::models::V2alpha1MetricStatus>) -> V2alpha1HorizontalPodAutoscalerStatus {
    self.current_metrics = current_metrics;
    self
  }

  pub fn current_metrics(&self) -> &Vec<::models::V2alpha1MetricStatus> {
    &self.current_metrics
  }


  pub fn set_current_replicas(&mut self, current_replicas: i32) {
    self.current_replicas = current_replicas;
  }

  pub fn with_current_replicas(mut self, current_replicas: i32) -> V2alpha1HorizontalPodAutoscalerStatus {
    self.current_replicas = current_replicas;
    self
  }

  pub fn current_replicas(&self) -> &i32 {
    &self.current_replicas
  }


  pub fn set_desired_replicas(&mut self, desired_replicas: i32) {
    self.desired_replicas = desired_replicas;
  }

  pub fn with_desired_replicas(mut self, desired_replicas: i32) -> V2alpha1HorizontalPodAutoscalerStatus {
    self.desired_replicas = desired_replicas;
    self
  }

  pub fn desired_replicas(&self) -> &i32 {
    &self.desired_replicas
  }


  pub fn set_last_scale_time(&mut self, last_scale_time: String) {
    self.last_scale_time = Some(last_scale_time);
  }

  pub fn with_last_scale_time(mut self, last_scale_time: String) -> V2alpha1HorizontalPodAutoscalerStatus {
    self.last_scale_time = Some(last_scale_time);
    self
  }

  pub fn last_scale_time(&self) -> Option<&String> {
    self.last_scale_time.as_ref()
  }

  pub fn reset_last_scale_time(&mut self) {
    self.last_scale_time = None;
  }

  pub fn set_observed_generation(&mut self, observed_generation: i64) {
    self.observed_generation = Some(observed_generation);
  }

  pub fn with_observed_generation(mut self, observed_generation: i64) -> V2alpha1HorizontalPodAutoscalerStatus {
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



