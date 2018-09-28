/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1DaemonSetUpdateStrategy {
  /// Rolling update config params. Present only if type = \"RollingUpdate\".
  #[serde(rename = "rollingUpdate")]
  rolling_update: Option<::models::V1beta1RollingUpdateDaemonSet>,
  /// Type of daemon set update. Can be \"RollingUpdate\" or \"OnDelete\". Default is OnDelete.
  #[serde(rename = "type")]
  _type: Option<String>
}

impl V1beta1DaemonSetUpdateStrategy {
  pub fn new() -> V1beta1DaemonSetUpdateStrategy {
    V1beta1DaemonSetUpdateStrategy {
      rolling_update: None,
      _type: None
    }
  }

  pub fn set_rolling_update(&mut self, rolling_update: ::models::V1beta1RollingUpdateDaemonSet) {
    self.rolling_update = Some(rolling_update);
  }

  pub fn with_rolling_update(mut self, rolling_update: ::models::V1beta1RollingUpdateDaemonSet) -> V1beta1DaemonSetUpdateStrategy {
    self.rolling_update = Some(rolling_update);
    self
  }

  pub fn rolling_update(&self) -> Option<&::models::V1beta1RollingUpdateDaemonSet> {
    self.rolling_update.as_ref()
  }

  pub fn reset_rolling_update(&mut self) {
    self.rolling_update = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> V1beta1DaemonSetUpdateStrategy {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



