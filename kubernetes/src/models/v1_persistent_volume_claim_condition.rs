/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PersistentVolumeClaimCondition : PersistentVolumeClaimCondition contails details about state of pvc

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1PersistentVolumeClaimCondition {
  /// Last time we probed the condition.
  #[serde(rename = "lastProbeTime")]
  last_probe_time: Option<String>,
  /// Last time the condition transitioned from one status to another.
  #[serde(rename = "lastTransitionTime")]
  last_transition_time: Option<String>,
  /// Human-readable message indicating details about last transition.
  #[serde(rename = "message")]
  message: Option<String>,
  /// Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports \"ResizeStarted\" that means the underlying persistent volume is being resized.
  #[serde(rename = "reason")]
  reason: Option<String>,
  #[serde(rename = "status")]
  status: String,
  #[serde(rename = "type")]
  _type: String
}

impl V1PersistentVolumeClaimCondition {
  /// PersistentVolumeClaimCondition contails details about state of pvc
  pub fn new(status: String, _type: String) -> V1PersistentVolumeClaimCondition {
    V1PersistentVolumeClaimCondition {
      last_probe_time: None,
      last_transition_time: None,
      message: None,
      reason: None,
      status: status,
      _type: _type
    }
  }

  pub fn set_last_probe_time(&mut self, last_probe_time: String) {
    self.last_probe_time = Some(last_probe_time);
  }

  pub fn with_last_probe_time(mut self, last_probe_time: String) -> V1PersistentVolumeClaimCondition {
    self.last_probe_time = Some(last_probe_time);
    self
  }

  pub fn last_probe_time(&self) -> Option<&String> {
    self.last_probe_time.as_ref()
  }

  pub fn reset_last_probe_time(&mut self) {
    self.last_probe_time = None;
  }

  pub fn set_last_transition_time(&mut self, last_transition_time: String) {
    self.last_transition_time = Some(last_transition_time);
  }

  pub fn with_last_transition_time(mut self, last_transition_time: String) -> V1PersistentVolumeClaimCondition {
    self.last_transition_time = Some(last_transition_time);
    self
  }

  pub fn last_transition_time(&self) -> Option<&String> {
    self.last_transition_time.as_ref()
  }

  pub fn reset_last_transition_time(&mut self) {
    self.last_transition_time = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> V1PersistentVolumeClaimCondition {
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

  pub fn with_reason(mut self, reason: String) -> V1PersistentVolumeClaimCondition {
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

  pub fn with_status(mut self, status: String) -> V1PersistentVolumeClaimCondition {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> V1PersistentVolumeClaimCondition {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



