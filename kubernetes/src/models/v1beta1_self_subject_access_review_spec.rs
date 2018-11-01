/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1SelfSubjectAccessReviewSpec : SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1SelfSubjectAccessReviewSpec {
  /// NonResourceAttributes describes information for a non-resource access request
  #[serde(rename = "nonResourceAttributes")]
  non_resource_attributes: Option<::models::V1beta1NonResourceAttributes>,
  /// ResourceAuthorizationAttributes describes information for a resource access request
  #[serde(rename = "resourceAttributes")]
  resource_attributes: Option<::models::V1beta1ResourceAttributes>
}

impl V1beta1SelfSubjectAccessReviewSpec {
  /// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
  pub fn new() -> V1beta1SelfSubjectAccessReviewSpec {
    V1beta1SelfSubjectAccessReviewSpec {
      non_resource_attributes: None,
      resource_attributes: None
    }
  }

  pub fn set_non_resource_attributes(&mut self, non_resource_attributes: ::models::V1beta1NonResourceAttributes) {
    self.non_resource_attributes = Some(non_resource_attributes);
  }

  pub fn with_non_resource_attributes(mut self, non_resource_attributes: ::models::V1beta1NonResourceAttributes) -> V1beta1SelfSubjectAccessReviewSpec {
    self.non_resource_attributes = Some(non_resource_attributes);
    self
  }

  pub fn non_resource_attributes(&self) -> Option<&::models::V1beta1NonResourceAttributes> {
    self.non_resource_attributes.as_ref()
  }

  pub fn reset_non_resource_attributes(&mut self) {
    self.non_resource_attributes = None;
  }

  pub fn set_resource_attributes(&mut self, resource_attributes: ::models::V1beta1ResourceAttributes) {
    self.resource_attributes = Some(resource_attributes);
  }

  pub fn with_resource_attributes(mut self, resource_attributes: ::models::V1beta1ResourceAttributes) -> V1beta1SelfSubjectAccessReviewSpec {
    self.resource_attributes = Some(resource_attributes);
    self
  }

  pub fn resource_attributes(&self) -> Option<&::models::V1beta1ResourceAttributes> {
    self.resource_attributes.as_ref()
  }

  pub fn reset_resource_attributes(&mut self) {
    self.resource_attributes = None;
  }

}



