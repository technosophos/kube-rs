/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AdmissionregistrationV1beta1ServiceReference : ServiceReference holds a reference to Service.legacy.k8s.io

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdmissionregistrationV1beta1ServiceReference {
  /// `name` is the name of the service. Required
  #[serde(rename = "name")]
  name: String,
  /// `namespace` is the namespace of the service. Required
  #[serde(rename = "namespace")]
  namespace: String,
  /// `path` is an optional URL path which will be sent in any request to this service.
  #[serde(rename = "path")]
  path: Option<String>
}

impl AdmissionregistrationV1beta1ServiceReference {
  /// ServiceReference holds a reference to Service.legacy.k8s.io
  pub fn new(name: String, namespace: String) -> AdmissionregistrationV1beta1ServiceReference {
    AdmissionregistrationV1beta1ServiceReference {
      name: name,
      namespace: namespace,
      path: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> AdmissionregistrationV1beta1ServiceReference {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_namespace(&mut self, namespace: String) {
    self.namespace = namespace;
  }

  pub fn with_namespace(mut self, namespace: String) -> AdmissionregistrationV1beta1ServiceReference {
    self.namespace = namespace;
    self
  }

  pub fn namespace(&self) -> &String {
    &self.namespace
  }


  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> AdmissionregistrationV1beta1ServiceReference {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

}



