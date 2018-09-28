/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1AdmissionHookClientConfig : AdmissionHookClientConfig contains the information to make a TLS connection with the webhook

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1AdmissionHookClientConfig {
  /// CABundle is a PEM encoded CA bundle which will be used to validate webhook's server certificate. Required
  #[serde(rename = "caBundle")]
  ca_bundle: String,
  /// Service is a reference to the service for this webhook. If there is only one port open for the service, that port will be used. If there are multiple ports open, port 443 will be used if it is open, otherwise it is an error. Required
  #[serde(rename = "service")]
  service: ::models::IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1ServiceReference
}

impl IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1AdmissionHookClientConfig {
  /// AdmissionHookClientConfig contains the information to make a TLS connection with the webhook
  pub fn new(ca_bundle: String, service: ::models::IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1ServiceReference) -> IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1AdmissionHookClientConfig {
    IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1AdmissionHookClientConfig {
      ca_bundle: ca_bundle,
      service: service
    }
  }

  pub fn set_ca_bundle(&mut self, ca_bundle: String) {
    self.ca_bundle = ca_bundle;
  }

  pub fn with_ca_bundle(mut self, ca_bundle: String) -> IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1AdmissionHookClientConfig {
    self.ca_bundle = ca_bundle;
    self
  }

  pub fn ca_bundle(&self) -> &String {
    &self.ca_bundle
  }


  pub fn set_service(&mut self, service: ::models::IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1ServiceReference) {
    self.service = service;
  }

  pub fn with_service(mut self, service: ::models::IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1ServiceReference) -> IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1AdmissionHookClientConfig {
    self.service = service;
    self
  }

  pub fn service(&self) -> &::models::IoK8sKubernetesPkgApisAdmissionregistrationV1alpha1ServiceReference {
    &self.service
  }


}



