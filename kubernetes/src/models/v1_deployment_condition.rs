/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1DeploymentCondition : DeploymentCondition describes the state of a deployment at a certain point.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1DeploymentCondition {
  /// Last time the condition transitioned from one status to another.
  #[serde(rename = "lastTransitionTime")]
  last_transition_time: Option<String>,
  /// The last time this condition was updated.
  #[serde(rename = "lastUpdateTime")]
  last_update_time: Option<String>,
  /// A human readable message indicating details about the transition.
  #[serde(rename = "message")]
  message: Option<String>,
  /// The reason for the condition's last transition.
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// Status of the condition, one of True, False, Unknown.
  #[serde(rename = "status")]
  status: String,
  /// Type of deployment condition.
  #[serde(rename = "type")]
  _type: String
}

impl V1DeploymentCondition {
  /// DeploymentCondition describes the state of a deployment at a certain point.
  pub fn new(status: String, _type: String) -> V1DeploymentCondition {
    V1DeploymentCondition {
      last_transition_time: None,
      last_update_time: None,
      message: None,
      reason: None,
      status: status,
      _type: _type
    }
  }

  pub fn set_last_transition_time(&mut self, last_transition_time: String) {
    self.last_transition_time = Some(last_transition_time);
  }

  pub fn with_last_transition_time(mut self, last_transition_time: String) -> V1DeploymentCondition {
    self.last_transition_time = Some(last_transition_time);
    self
  }

  pub fn last_transition_time(&self) -> Option<&String> {
    self.last_transition_time.as_ref()
  }

  pub fn reset_last_transition_time(&mut self) {
    self.last_transition_time = None;
  }

  pub fn set_last_update_time(&mut self, last_update_time: String) {
    self.last_update_time = Some(last_update_time);
  }

  pub fn with_last_update_time(mut self, last_update_time: String) -> V1DeploymentCondition {
    self.last_update_time = Some(last_update_time);
    self
  }

  pub fn last_update_time(&self) -> Option<&String> {
    self.last_update_time.as_ref()
  }

  pub fn reset_last_update_time(&mut self) {
    self.last_update_time = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> V1DeploymentCondition {
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

  pub fn with_reason(mut self, reason: String) -> V1DeploymentCondition {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> V1DeploymentCondition {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> V1DeploymentCondition {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



