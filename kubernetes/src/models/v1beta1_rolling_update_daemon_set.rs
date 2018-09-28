/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1RollingUpdateDaemonSet : Spec to control the desired behavior of daemon set rolling update.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1RollingUpdateDaemonSet {
  /// The maximum number of DaemonSet pods that can be unavailable during the update. ::serde_json::Value can be an absolute number (ex: 5) or a percentage of total number of DaemonSet pods at the start of the update (ex: 10%). Absolute number is calculated from percentage by rounding up. This cannot be 0. Default value is 1. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their pods stopped for an update at any given time. The update starts by stopping at most 30% of those DaemonSet pods and then brings up new DaemonSet pods in their place. Once the new pods are available, it then proceeds onto other DaemonSet pods, thus ensuring that at least 70% of original number of DaemonSet pods are available at all times during the update.
  #[serde(rename = "maxUnavailable")]
  max_unavailable: Option<::serde_json::Value>
}

impl V1beta1RollingUpdateDaemonSet {
  /// Spec to control the desired behavior of daemon set rolling update.
  pub fn new() -> V1beta1RollingUpdateDaemonSet {
    V1beta1RollingUpdateDaemonSet {
      max_unavailable: None
    }
  }

  pub fn set_max_unavailable(&mut self, max_unavailable: ::serde_json::Value) {
    self.max_unavailable = Some(max_unavailable);
  }

  pub fn with_max_unavailable(mut self, max_unavailable: ::serde_json::Value) -> V1beta1RollingUpdateDaemonSet {
    self.max_unavailable = Some(max_unavailable);
    self
  }

  pub fn max_unavailable(&self) -> Option<&Value> {
    self.max_unavailable.as_ref()
  }

  pub fn reset_max_unavailable(&mut self) {
    self.max_unavailable = None;
  }

}



