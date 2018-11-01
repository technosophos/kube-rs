/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1NodeStatus : NodeStatus is information about the current status of a node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1NodeStatus {
  /// List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses
  #[serde(rename = "addresses")]
  addresses: Option<Vec<::models::V1NodeAddress>>,
  /// Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
  #[serde(rename = "allocatable")]
  allocatable: Option<::std::collections::HashMap<String, String>>,
  /// Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
  #[serde(rename = "capacity")]
  capacity: Option<::std::collections::HashMap<String, String>>,
  /// Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::V1NodeCondition>>,
  /// Status of the config assigned to the node via the dynamic Kubelet config feature.
  #[serde(rename = "config")]
  config: Option<::models::V1NodeConfigStatus>,
  /// Endpoints of daemons running on the Node.
  #[serde(rename = "daemonEndpoints")]
  daemon_endpoints: Option<::models::V1NodeDaemonEndpoints>,
  /// List of container images on this node
  #[serde(rename = "images")]
  images: Option<Vec<::models::V1ContainerImage>>,
  /// Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#info
  #[serde(rename = "nodeInfo")]
  node_info: Option<::models::V1NodeSystemInfo>,
  /// NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
  #[serde(rename = "phase")]
  phase: Option<String>,
  /// List of volumes that are attached to the node.
  #[serde(rename = "volumesAttached")]
  volumes_attached: Option<Vec<::models::V1AttachedVolume>>,
  /// List of attachable volumes in use (mounted) by the node.
  #[serde(rename = "volumesInUse")]
  volumes_in_use: Option<Vec<String>>
}

impl V1NodeStatus {
  /// NodeStatus is information about the current status of a node.
  pub fn new() -> V1NodeStatus {
    V1NodeStatus {
      addresses: None,
      allocatable: None,
      capacity: None,
      conditions: None,
      config: None,
      daemon_endpoints: None,
      images: None,
      node_info: None,
      phase: None,
      volumes_attached: None,
      volumes_in_use: None
    }
  }

  pub fn set_addresses(&mut self, addresses: Vec<::models::V1NodeAddress>) {
    self.addresses = Some(addresses);
  }

  pub fn with_addresses(mut self, addresses: Vec<::models::V1NodeAddress>) -> V1NodeStatus {
    self.addresses = Some(addresses);
    self
  }

  pub fn addresses(&self) -> Option<&Vec<::models::V1NodeAddress>> {
    self.addresses.as_ref()
  }

  pub fn reset_addresses(&mut self) {
    self.addresses = None;
  }

  pub fn set_allocatable(&mut self, allocatable: ::std::collections::HashMap<String, String>) {
    self.allocatable = Some(allocatable);
  }

  pub fn with_allocatable(mut self, allocatable: ::std::collections::HashMap<String, String>) -> V1NodeStatus {
    self.allocatable = Some(allocatable);
    self
  }

  pub fn allocatable(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.allocatable.as_ref()
  }

  pub fn reset_allocatable(&mut self) {
    self.allocatable = None;
  }

  pub fn set_capacity(&mut self, capacity: ::std::collections::HashMap<String, String>) {
    self.capacity = Some(capacity);
  }

  pub fn with_capacity(mut self, capacity: ::std::collections::HashMap<String, String>) -> V1NodeStatus {
    self.capacity = Some(capacity);
    self
  }

  pub fn capacity(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.capacity.as_ref()
  }

  pub fn reset_capacity(&mut self) {
    self.capacity = None;
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::V1NodeCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::V1NodeCondition>) -> V1NodeStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::V1NodeCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_config(&mut self, config: ::models::V1NodeConfigStatus) {
    self.config = Some(config);
  }

  pub fn with_config(mut self, config: ::models::V1NodeConfigStatus) -> V1NodeStatus {
    self.config = Some(config);
    self
  }

  pub fn config(&self) -> Option<&::models::V1NodeConfigStatus> {
    self.config.as_ref()
  }

  pub fn reset_config(&mut self) {
    self.config = None;
  }

  pub fn set_daemon_endpoints(&mut self, daemon_endpoints: ::models::V1NodeDaemonEndpoints) {
    self.daemon_endpoints = Some(daemon_endpoints);
  }

  pub fn with_daemon_endpoints(mut self, daemon_endpoints: ::models::V1NodeDaemonEndpoints) -> V1NodeStatus {
    self.daemon_endpoints = Some(daemon_endpoints);
    self
  }

  pub fn daemon_endpoints(&self) -> Option<&::models::V1NodeDaemonEndpoints> {
    self.daemon_endpoints.as_ref()
  }

  pub fn reset_daemon_endpoints(&mut self) {
    self.daemon_endpoints = None;
  }

  pub fn set_images(&mut self, images: Vec<::models::V1ContainerImage>) {
    self.images = Some(images);
  }

  pub fn with_images(mut self, images: Vec<::models::V1ContainerImage>) -> V1NodeStatus {
    self.images = Some(images);
    self
  }

  pub fn images(&self) -> Option<&Vec<::models::V1ContainerImage>> {
    self.images.as_ref()
  }

  pub fn reset_images(&mut self) {
    self.images = None;
  }

  pub fn set_node_info(&mut self, node_info: ::models::V1NodeSystemInfo) {
    self.node_info = Some(node_info);
  }

  pub fn with_node_info(mut self, node_info: ::models::V1NodeSystemInfo) -> V1NodeStatus {
    self.node_info = Some(node_info);
    self
  }

  pub fn node_info(&self) -> Option<&::models::V1NodeSystemInfo> {
    self.node_info.as_ref()
  }

  pub fn reset_node_info(&mut self) {
    self.node_info = None;
  }

  pub fn set_phase(&mut self, phase: String) {
    self.phase = Some(phase);
  }

  pub fn with_phase(mut self, phase: String) -> V1NodeStatus {
    self.phase = Some(phase);
    self
  }

  pub fn phase(&self) -> Option<&String> {
    self.phase.as_ref()
  }

  pub fn reset_phase(&mut self) {
    self.phase = None;
  }

  pub fn set_volumes_attached(&mut self, volumes_attached: Vec<::models::V1AttachedVolume>) {
    self.volumes_attached = Some(volumes_attached);
  }

  pub fn with_volumes_attached(mut self, volumes_attached: Vec<::models::V1AttachedVolume>) -> V1NodeStatus {
    self.volumes_attached = Some(volumes_attached);
    self
  }

  pub fn volumes_attached(&self) -> Option<&Vec<::models::V1AttachedVolume>> {
    self.volumes_attached.as_ref()
  }

  pub fn reset_volumes_attached(&mut self) {
    self.volumes_attached = None;
  }

  pub fn set_volumes_in_use(&mut self, volumes_in_use: Vec<String>) {
    self.volumes_in_use = Some(volumes_in_use);
  }

  pub fn with_volumes_in_use(mut self, volumes_in_use: Vec<String>) -> V1NodeStatus {
    self.volumes_in_use = Some(volumes_in_use);
    self
  }

  pub fn volumes_in_use(&self) -> Option<&Vec<String>> {
    self.volumes_in_use.as_ref()
  }

  pub fn reset_volumes_in_use(&mut self) {
    self.volumes_in_use = None;
  }

}



