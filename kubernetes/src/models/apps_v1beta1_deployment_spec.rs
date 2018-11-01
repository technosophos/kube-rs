/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AppsV1beta1DeploymentSpec : DeploymentSpec is the specification of the desired behavior of the Deployment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppsV1beta1DeploymentSpec {
  /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
  #[serde(rename = "minReadySeconds")]
  min_ready_seconds: Option<i32>,
  /// Indicates that the deployment is paused.
  #[serde(rename = "paused")]
  paused: Option<bool>,
  /// The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. Defaults to 600s.
  #[serde(rename = "progressDeadlineSeconds")]
  progress_deadline_seconds: Option<i32>,
  /// Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
  #[serde(rename = "replicas")]
  replicas: Option<i32>,
  /// The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 2.
  #[serde(rename = "revisionHistoryLimit")]
  revision_history_limit: Option<i32>,
  /// DEPRECATED. The config this deployment is rolling back to. Will be cleared after rollback is done.
  #[serde(rename = "rollbackTo")]
  rollback_to: Option<::models::AppsV1beta1RollbackConfig>,
  /// Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment.
  #[serde(rename = "selector")]
  selector: Option<::models::V1LabelSelector>,
  /// The deployment strategy to use to replace existing pods with new ones.
  #[serde(rename = "strategy")]
  strategy: Option<::models::AppsV1beta1DeploymentStrategy>,
  /// Template describes the pods that will be created.
  #[serde(rename = "template")]
  template: ::models::V1PodTemplateSpec
}

impl AppsV1beta1DeploymentSpec {
  /// DeploymentSpec is the specification of the desired behavior of the Deployment.
  pub fn new(template: ::models::V1PodTemplateSpec) -> AppsV1beta1DeploymentSpec {
    AppsV1beta1DeploymentSpec {
      min_ready_seconds: None,
      paused: None,
      progress_deadline_seconds: None,
      replicas: None,
      revision_history_limit: None,
      rollback_to: None,
      selector: None,
      strategy: None,
      template: template
    }
  }

  pub fn set_min_ready_seconds(&mut self, min_ready_seconds: i32) {
    self.min_ready_seconds = Some(min_ready_seconds);
  }

  pub fn with_min_ready_seconds(mut self, min_ready_seconds: i32) -> AppsV1beta1DeploymentSpec {
    self.min_ready_seconds = Some(min_ready_seconds);
    self
  }

  pub fn min_ready_seconds(&self) -> Option<&i32> {
    self.min_ready_seconds.as_ref()
  }

  pub fn reset_min_ready_seconds(&mut self) {
    self.min_ready_seconds = None;
  }

  pub fn set_paused(&mut self, paused: bool) {
    self.paused = Some(paused);
  }

  pub fn with_paused(mut self, paused: bool) -> AppsV1beta1DeploymentSpec {
    self.paused = Some(paused);
    self
  }

  pub fn paused(&self) -> Option<&bool> {
    self.paused.as_ref()
  }

  pub fn reset_paused(&mut self) {
    self.paused = None;
  }

  pub fn set_progress_deadline_seconds(&mut self, progress_deadline_seconds: i32) {
    self.progress_deadline_seconds = Some(progress_deadline_seconds);
  }

  pub fn with_progress_deadline_seconds(mut self, progress_deadline_seconds: i32) -> AppsV1beta1DeploymentSpec {
    self.progress_deadline_seconds = Some(progress_deadline_seconds);
    self
  }

  pub fn progress_deadline_seconds(&self) -> Option<&i32> {
    self.progress_deadline_seconds.as_ref()
  }

  pub fn reset_progress_deadline_seconds(&mut self) {
    self.progress_deadline_seconds = None;
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = Some(replicas);
  }

  pub fn with_replicas(mut self, replicas: i32) -> AppsV1beta1DeploymentSpec {
    self.replicas = Some(replicas);
    self
  }

  pub fn replicas(&self) -> Option<&i32> {
    self.replicas.as_ref()
  }

  pub fn reset_replicas(&mut self) {
    self.replicas = None;
  }

  pub fn set_revision_history_limit(&mut self, revision_history_limit: i32) {
    self.revision_history_limit = Some(revision_history_limit);
  }

  pub fn with_revision_history_limit(mut self, revision_history_limit: i32) -> AppsV1beta1DeploymentSpec {
    self.revision_history_limit = Some(revision_history_limit);
    self
  }

  pub fn revision_history_limit(&self) -> Option<&i32> {
    self.revision_history_limit.as_ref()
  }

  pub fn reset_revision_history_limit(&mut self) {
    self.revision_history_limit = None;
  }

  pub fn set_rollback_to(&mut self, rollback_to: ::models::AppsV1beta1RollbackConfig) {
    self.rollback_to = Some(rollback_to);
  }

  pub fn with_rollback_to(mut self, rollback_to: ::models::AppsV1beta1RollbackConfig) -> AppsV1beta1DeploymentSpec {
    self.rollback_to = Some(rollback_to);
    self
  }

  pub fn rollback_to(&self) -> Option<&::models::AppsV1beta1RollbackConfig> {
    self.rollback_to.as_ref()
  }

  pub fn reset_rollback_to(&mut self) {
    self.rollback_to = None;
  }

  pub fn set_selector(&mut self, selector: ::models::V1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::V1LabelSelector) -> AppsV1beta1DeploymentSpec {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::V1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

  pub fn set_strategy(&mut self, strategy: ::models::AppsV1beta1DeploymentStrategy) {
    self.strategy = Some(strategy);
  }

  pub fn with_strategy(mut self, strategy: ::models::AppsV1beta1DeploymentStrategy) -> AppsV1beta1DeploymentSpec {
    self.strategy = Some(strategy);
    self
  }

  pub fn strategy(&self) -> Option<&::models::AppsV1beta1DeploymentStrategy> {
    self.strategy.as_ref()
  }

  pub fn reset_strategy(&mut self) {
    self.strategy = None;
  }

  pub fn set_template(&mut self, template: ::models::V1PodTemplateSpec) {
    self.template = template;
  }

  pub fn with_template(mut self, template: ::models::V1PodTemplateSpec) -> AppsV1beta1DeploymentSpec {
    self.template = template;
    self
  }

  pub fn template(&self) -> &::models::V1PodTemplateSpec {
    &self.template
  }


}



