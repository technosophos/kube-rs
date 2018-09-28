/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ContainerState : ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ContainerState {
  /// Details about a running container
  #[serde(rename = "running")]
  running: Option<::models::V1ContainerStateRunning>,
  /// Details about a terminated container
  #[serde(rename = "terminated")]
  terminated: Option<::models::V1ContainerStateTerminated>,
  /// Details about a waiting container
  #[serde(rename = "waiting")]
  waiting: Option<::models::V1ContainerStateWaiting>
}

impl V1ContainerState {
  /// ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
  pub fn new() -> V1ContainerState {
    V1ContainerState {
      running: None,
      terminated: None,
      waiting: None
    }
  }

  pub fn set_running(&mut self, running: ::models::V1ContainerStateRunning) {
    self.running = Some(running);
  }

  pub fn with_running(mut self, running: ::models::V1ContainerStateRunning) -> V1ContainerState {
    self.running = Some(running);
    self
  }

  pub fn running(&self) -> Option<&::models::V1ContainerStateRunning> {
    self.running.as_ref()
  }

  pub fn reset_running(&mut self) {
    self.running = None;
  }

  pub fn set_terminated(&mut self, terminated: ::models::V1ContainerStateTerminated) {
    self.terminated = Some(terminated);
  }

  pub fn with_terminated(mut self, terminated: ::models::V1ContainerStateTerminated) -> V1ContainerState {
    self.terminated = Some(terminated);
    self
  }

  pub fn terminated(&self) -> Option<&::models::V1ContainerStateTerminated> {
    self.terminated.as_ref()
  }

  pub fn reset_terminated(&mut self) {
    self.terminated = None;
  }

  pub fn set_waiting(&mut self, waiting: ::models::V1ContainerStateWaiting) {
    self.waiting = Some(waiting);
  }

  pub fn with_waiting(mut self, waiting: ::models::V1ContainerStateWaiting) -> V1ContainerState {
    self.waiting = Some(waiting);
    self
  }

  pub fn waiting(&self) -> Option<&::models::V1ContainerStateWaiting> {
    self.waiting.as_ref()
  }

  pub fn reset_waiting(&mut self) {
    self.waiting = None;
  }

}



