/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApiV1HostAlias : HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApiV1HostAlias {
  /// Hostnames for the above IP address.
  #[serde(rename = "hostnames")]
  hostnames: Option<Vec<String>>,
  /// IP address of the host file entry.
  #[serde(rename = "ip")]
  ip: Option<String>
}

impl IoK8sKubernetesPkgApiV1HostAlias {
  /// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
  pub fn new() -> IoK8sKubernetesPkgApiV1HostAlias {
    IoK8sKubernetesPkgApiV1HostAlias {
      hostnames: None,
      ip: None
    }
  }

  pub fn set_hostnames(&mut self, hostnames: Vec<String>) {
    self.hostnames = Some(hostnames);
  }

  pub fn with_hostnames(mut self, hostnames: Vec<String>) -> IoK8sKubernetesPkgApiV1HostAlias {
    self.hostnames = Some(hostnames);
    self
  }

  pub fn hostnames(&self) -> Option<&Vec<String>> {
    self.hostnames.as_ref()
  }

  pub fn reset_hostnames(&mut self) {
    self.hostnames = None;
  }

  pub fn set_ip(&mut self, ip: String) {
    self.ip = Some(ip);
  }

  pub fn with_ip(mut self, ip: String) -> IoK8sKubernetesPkgApiV1HostAlias {
    self.ip = Some(ip);
    self
  }

  pub fn ip(&self) -> Option<&String> {
    self.ip.as_ref()
  }

  pub fn reset_ip(&mut self) {
    self.ip = None;
  }

}



