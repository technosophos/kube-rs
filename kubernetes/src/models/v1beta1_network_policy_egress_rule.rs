/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1NetworkPolicyEgressRule : DEPRECATED 1.9 - This group version of NetworkPolicyEgressRule is deprecated by networking/v1/NetworkPolicyEgressRule. NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to. This type is beta-level in 1.8

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1NetworkPolicyEgressRule {
  /// List of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
  #[serde(rename = "ports")]
  ports: Option<Vec<::models::V1beta1NetworkPolicyPort>>,
  /// List of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all destinations (traffic not restricted by destination). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the to list.
  #[serde(rename = "to")]
  to: Option<Vec<::models::V1beta1NetworkPolicyPeer>>
}

impl V1beta1NetworkPolicyEgressRule {
  /// DEPRECATED 1.9 - This group version of NetworkPolicyEgressRule is deprecated by networking/v1/NetworkPolicyEgressRule. NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to. This type is beta-level in 1.8
  pub fn new() -> V1beta1NetworkPolicyEgressRule {
    V1beta1NetworkPolicyEgressRule {
      ports: None,
      to: None
    }
  }

  pub fn set_ports(&mut self, ports: Vec<::models::V1beta1NetworkPolicyPort>) {
    self.ports = Some(ports);
  }

  pub fn with_ports(mut self, ports: Vec<::models::V1beta1NetworkPolicyPort>) -> V1beta1NetworkPolicyEgressRule {
    self.ports = Some(ports);
    self
  }

  pub fn ports(&self) -> Option<&Vec<::models::V1beta1NetworkPolicyPort>> {
    self.ports.as_ref()
  }

  pub fn reset_ports(&mut self) {
    self.ports = None;
  }

  pub fn set_to(&mut self, to: Vec<::models::V1beta1NetworkPolicyPeer>) {
    self.to = Some(to);
  }

  pub fn with_to(mut self, to: Vec<::models::V1beta1NetworkPolicyPeer>) -> V1beta1NetworkPolicyEgressRule {
    self.to = Some(to);
    self
  }

  pub fn to(&self) -> Option<&Vec<::models::V1beta1NetworkPolicyPeer>> {
    self.to.as_ref()
  }

  pub fn reset_to(&mut self) {
    self.to = None;
  }

}



