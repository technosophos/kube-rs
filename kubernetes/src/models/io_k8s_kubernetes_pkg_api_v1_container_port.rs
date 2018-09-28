/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1ContainerPort : ContainerPort represents a network port in a single container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1ContainerPort {
  /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
  #[serde(rename = "containerPort")]
  container_port: i32,
  /// What host IP to bind the external port to.
  #[serde(rename = "hostIP")]
  host_ip: Option<String>,
  /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
  #[serde(rename = "hostPort")]
  host_port: Option<i32>,
  /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Protocol for port. Must be UDP or TCP. Defaults to \"TCP\".
  #[serde(rename = "protocol")]
  protocol: Option<String>
}

impl IoK8sKubernetesPkgApiV1ContainerPort {
  /// ContainerPort represents a network port in a single container.
  pub fn new(container_port: i32) -> IoK8sKubernetesPkgApiV1ContainerPort {
    IoK8sKubernetesPkgApiV1ContainerPort {
      container_port: container_port,
      host_ip: None,
      host_port: None,
      name: None,
      protocol: None
    }
  }

  pub fn set_container_port(&mut self, container_port: i32) {
    self.container_port = container_port;
  }

  pub fn with_container_port(mut self, container_port: i32) -> IoK8sKubernetesPkgApiV1ContainerPort {
    self.container_port = container_port;
    self
  }

  pub fn container_port(&self) -> &i32 {
    &self.container_port
  }


  pub fn set_host_ip(&mut self, host_ip: String) {
    self.host_ip = Some(host_ip);
  }

  pub fn with_host_ip(mut self, host_ip: String) -> IoK8sKubernetesPkgApiV1ContainerPort {
    self.host_ip = Some(host_ip);
    self
  }

  pub fn host_ip(&self) -> Option<&String> {
    self.host_ip.as_ref()
  }

  pub fn reset_host_ip(&mut self) {
    self.host_ip = None;
  }

  pub fn set_host_port(&mut self, host_port: i32) {
    self.host_port = Some(host_port);
  }

  pub fn with_host_port(mut self, host_port: i32) -> IoK8sKubernetesPkgApiV1ContainerPort {
    self.host_port = Some(host_port);
    self
  }

  pub fn host_port(&self) -> Option<&i32> {
    self.host_port.as_ref()
  }

  pub fn reset_host_port(&mut self) {
    self.host_port = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> IoK8sKubernetesPkgApiV1ContainerPort {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_protocol(&mut self, protocol: String) {
    self.protocol = Some(protocol);
  }

  pub fn with_protocol(mut self, protocol: String) -> IoK8sKubernetesPkgApiV1ContainerPort {
    self.protocol = Some(protocol);
    self
  }

  pub fn protocol(&self) -> Option<&String> {
    self.protocol.as_ref()
  }

  pub fn reset_protocol(&mut self) {
    self.protocol = None;
  }

}



