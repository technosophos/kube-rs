/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisExtensionsV1beta1IngressBackend : IngressBackend describes all endpoints for a given service and port.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisExtensionsV1beta1IngressBackend {
  /// Specifies the name of the referenced service.
  #[serde(rename = "serviceName")]
  service_name: String,
  /// Specifies the port of the referenced service.
  #[serde(rename = "servicePort")]
  service_port: ::models::IoK8sApimachineryPkgUtilIntstrIntOrString
}

impl IoK8sKubernetesPkgApisExtensionsV1beta1IngressBackend {
  /// IngressBackend describes all endpoints for a given service and port.
  pub fn new(service_name: String, service_port: ::models::IoK8sApimachineryPkgUtilIntstrIntOrString) -> IoK8sKubernetesPkgApisExtensionsV1beta1IngressBackend {
    IoK8sKubernetesPkgApisExtensionsV1beta1IngressBackend {
      service_name: service_name,
      service_port: service_port
    }
  }

  pub fn set_service_name(&mut self, service_name: String) {
    self.service_name = service_name;
  }

  pub fn with_service_name(mut self, service_name: String) -> IoK8sKubernetesPkgApisExtensionsV1beta1IngressBackend {
    self.service_name = service_name;
    self
  }

  pub fn service_name(&self) -> &String {
    &self.service_name
  }


  pub fn set_service_port(&mut self, service_port: ::models::IoK8sApimachineryPkgUtilIntstrIntOrString) {
    self.service_port = service_port;
  }

  pub fn with_service_port(mut self, service_port: ::models::IoK8sApimachineryPkgUtilIntstrIntOrString) -> IoK8sKubernetesPkgApisExtensionsV1beta1IngressBackend {
    self.service_port = service_port;
    self
  }

  pub fn service_port(&self) -> &::models::IoK8sApimachineryPkgUtilIntstrIntOrString {
    &self.service_port
  }


}



