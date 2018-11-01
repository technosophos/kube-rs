/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1StatefulSetUpdateStrategy : StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1StatefulSetUpdateStrategy {
  /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
  #[serde(rename = "rollingUpdate")]
  rolling_update: Option<::models::V1RollingUpdateStatefulSetStrategy>,
  /// Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.
  #[serde(rename = "type")]
  _type: Option<String>
}

impl V1StatefulSetUpdateStrategy {
  /// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
  pub fn new() -> V1StatefulSetUpdateStrategy {
    V1StatefulSetUpdateStrategy {
      rolling_update: None,
      _type: None
    }
  }

  pub fn set_rolling_update(&mut self, rolling_update: ::models::V1RollingUpdateStatefulSetStrategy) {
    self.rolling_update = Some(rolling_update);
  }

  pub fn with_rolling_update(mut self, rolling_update: ::models::V1RollingUpdateStatefulSetStrategy) -> V1StatefulSetUpdateStrategy {
    self.rolling_update = Some(rolling_update);
    self
  }

  pub fn rolling_update(&self) -> Option<&::models::V1RollingUpdateStatefulSetStrategy> {
    self.rolling_update.as_ref()
  }

  pub fn reset_rolling_update(&mut self) {
    self.rolling_update = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> V1StatefulSetUpdateStrategy {
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



