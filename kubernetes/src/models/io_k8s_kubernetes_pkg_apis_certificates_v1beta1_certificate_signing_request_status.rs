/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestStatus {
  /// If request was approved, the controller will place the issued certificate here.
  #[serde(rename = "certificate")]
  certificate: Option<String>,
  /// Conditions applied to the request, such as approval or denial.
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestCondition>>
}

impl IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestStatus {
  pub fn new() -> IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestStatus {
    IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestStatus {
      certificate: None,
      conditions: None
    }
  }

  pub fn set_certificate(&mut self, certificate: String) {
    self.certificate = Some(certificate);
  }

  pub fn with_certificate(mut self, certificate: String) -> IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestStatus {
    self.certificate = Some(certificate);
    self
  }

  pub fn certificate(&self) -> Option<&String> {
    self.certificate.as_ref()
  }

  pub fn reset_certificate(&mut self) {
    self.certificate = None;
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestCondition>) -> IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::IoK8sKubernetesPkgApisCertificatesV1beta1CertificateSigningRequestCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

}



