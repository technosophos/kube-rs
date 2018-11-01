/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PodAntiAffinity : Pod anti affinity is a group of inter pod anti affinity scheduling rules.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1PodAntiAffinity {
  /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding \"weight\" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
  #[serde(rename = "preferredDuringSchedulingIgnoredDuringExecution")]
  preferred_during_scheduling_ignored_during_execution: Option<Vec<::models::V1WeightedPodAffinityTerm>>,
  /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
  #[serde(rename = "requiredDuringSchedulingIgnoredDuringExecution")]
  required_during_scheduling_ignored_during_execution: Option<Vec<::models::V1PodAffinityTerm>>
}

impl V1PodAntiAffinity {
  /// Pod anti affinity is a group of inter pod anti affinity scheduling rules.
  pub fn new() -> V1PodAntiAffinity {
    V1PodAntiAffinity {
      preferred_during_scheduling_ignored_during_execution: None,
      required_during_scheduling_ignored_during_execution: None
    }
  }

  pub fn set_preferred_during_scheduling_ignored_during_execution(&mut self, preferred_during_scheduling_ignored_during_execution: Vec<::models::V1WeightedPodAffinityTerm>) {
    self.preferred_during_scheduling_ignored_during_execution = Some(preferred_during_scheduling_ignored_during_execution);
  }

  pub fn with_preferred_during_scheduling_ignored_during_execution(mut self, preferred_during_scheduling_ignored_during_execution: Vec<::models::V1WeightedPodAffinityTerm>) -> V1PodAntiAffinity {
    self.preferred_during_scheduling_ignored_during_execution = Some(preferred_during_scheduling_ignored_during_execution);
    self
  }

  pub fn preferred_during_scheduling_ignored_during_execution(&self) -> Option<&Vec<::models::V1WeightedPodAffinityTerm>> {
    self.preferred_during_scheduling_ignored_during_execution.as_ref()
  }

  pub fn reset_preferred_during_scheduling_ignored_during_execution(&mut self) {
    self.preferred_during_scheduling_ignored_during_execution = None;
  }

  pub fn set_required_during_scheduling_ignored_during_execution(&mut self, required_during_scheduling_ignored_during_execution: Vec<::models::V1PodAffinityTerm>) {
    self.required_during_scheduling_ignored_during_execution = Some(required_during_scheduling_ignored_during_execution);
  }

  pub fn with_required_during_scheduling_ignored_during_execution(mut self, required_during_scheduling_ignored_during_execution: Vec<::models::V1PodAffinityTerm>) -> V1PodAntiAffinity {
    self.required_during_scheduling_ignored_during_execution = Some(required_during_scheduling_ignored_during_execution);
    self
  }

  pub fn required_during_scheduling_ignored_during_execution(&self) -> Option<&Vec<::models::V1PodAffinityTerm>> {
    self.required_during_scheduling_ignored_during_execution.as_ref()
  }

  pub fn reset_required_during_scheduling_ignored_during_execution(&mut self) {
    self.required_during_scheduling_ignored_during_execution = None;
  }

}



