/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ContainerStateTerminated : ContainerStateTerminated is a terminated state of a container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ContainerStateTerminated {
  /// Container's ID in the format 'docker://<container_id>'
  #[serde(rename = "containerID")]
  container_id: Option<String>,
  /// Exit status from the last termination of the container
  #[serde(rename = "exitCode")]
  exit_code: i32,
  /// Time at which the container last terminated
  #[serde(rename = "finishedAt")]
  finished_at: Option<String>,
  /// Message regarding the last termination of the container
  #[serde(rename = "message")]
  message: Option<String>,
  /// (brief) reason from the last termination of the container
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// Signal from the last termination of the container
  #[serde(rename = "signal")]
  signal: Option<i32>,
  /// Time at which previous execution of the container started
  #[serde(rename = "startedAt")]
  started_at: Option<String>
}

impl V1ContainerStateTerminated {
  /// ContainerStateTerminated is a terminated state of a container.
  pub fn new(exit_code: i32) -> V1ContainerStateTerminated {
    V1ContainerStateTerminated {
      container_id: None,
      exit_code: exit_code,
      finished_at: None,
      message: None,
      reason: None,
      signal: None,
      started_at: None
    }
  }

  pub fn set_container_id(&mut self, container_id: String) {
    self.container_id = Some(container_id);
  }

  pub fn with_container_id(mut self, container_id: String) -> V1ContainerStateTerminated {
    self.container_id = Some(container_id);
    self
  }

  pub fn container_id(&self) -> Option<&String> {
    self.container_id.as_ref()
  }

  pub fn reset_container_id(&mut self) {
    self.container_id = None;
  }

  pub fn set_exit_code(&mut self, exit_code: i32) {
    self.exit_code = exit_code;
  }

  pub fn with_exit_code(mut self, exit_code: i32) -> V1ContainerStateTerminated {
    self.exit_code = exit_code;
    self
  }

  pub fn exit_code(&self) -> &i32 {
    &self.exit_code
  }


  pub fn set_finished_at(&mut self, finished_at: String) {
    self.finished_at = Some(finished_at);
  }

  pub fn with_finished_at(mut self, finished_at: String) -> V1ContainerStateTerminated {
    self.finished_at = Some(finished_at);
    self
  }

  pub fn finished_at(&self) -> Option<&String> {
    self.finished_at.as_ref()
  }

  pub fn reset_finished_at(&mut self) {
    self.finished_at = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> V1ContainerStateTerminated {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> V1ContainerStateTerminated {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_signal(&mut self, signal: i32) {
    self.signal = Some(signal);
  }

  pub fn with_signal(mut self, signal: i32) -> V1ContainerStateTerminated {
    self.signal = Some(signal);
    self
  }

  pub fn signal(&self) -> Option<&i32> {
    self.signal.as_ref()
  }

  pub fn reset_signal(&mut self) {
    self.signal = None;
  }

  pub fn set_started_at(&mut self, started_at: String) {
    self.started_at = Some(started_at);
  }

  pub fn with_started_at(mut self, started_at: String) -> V1ContainerStateTerminated {
    self.started_at = Some(started_at);
    self
  }

  pub fn started_at(&self) -> Option<&String> {
    self.started_at.as_ref()
  }

  pub fn reset_started_at(&mut self) {
    self.started_at = None;
  }

}



