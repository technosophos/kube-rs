/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1SubjectAccessReviewSpec : SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1SubjectAccessReviewSpec {
  /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
  #[serde(rename = "extra")]
  extra: Option<::std::collections::HashMap<String, Vec<String>>>,
  /// Groups is the groups you're testing for.
  #[serde(rename = "group")]
  group: Option<Vec<String>>,
  /// NonResourceAttributes describes information for a non-resource access request
  #[serde(rename = "nonResourceAttributes")]
  non_resource_attributes: Option<::models::V1beta1NonResourceAttributes>,
  /// ResourceAuthorizationAttributes describes information for a resource access request
  #[serde(rename = "resourceAttributes")]
  resource_attributes: Option<::models::V1beta1ResourceAttributes>,
  /// UID information about the requesting user.
  #[serde(rename = "uid")]
  uid: Option<String>,
  /// User is the user you're testing for. If you specify \"User\" but not \"Group\", then is it interpreted as \"What if User were not a member of any groups
  #[serde(rename = "user")]
  user: Option<String>
}

impl V1beta1SubjectAccessReviewSpec {
  /// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
  pub fn new() -> V1beta1SubjectAccessReviewSpec {
    V1beta1SubjectAccessReviewSpec {
      extra: None,
      group: None,
      non_resource_attributes: None,
      resource_attributes: None,
      uid: None,
      user: None
    }
  }

  pub fn set_extra(&mut self, extra: ::std::collections::HashMap<String, Vec<String>>) {
    self.extra = Some(extra);
  }

  pub fn with_extra(mut self, extra: ::std::collections::HashMap<String, Vec<String>>) -> V1beta1SubjectAccessReviewSpec {
    self.extra = Some(extra);
    self
  }

  pub fn extra(&self) -> Option<&::std::collections::HashMap<String, Vec<String>>> {
    self.extra.as_ref()
  }

  pub fn reset_extra(&mut self) {
    self.extra = None;
  }

  pub fn set_group(&mut self, group: Vec<String>) {
    self.group = Some(group);
  }

  pub fn with_group(mut self, group: Vec<String>) -> V1beta1SubjectAccessReviewSpec {
    self.group = Some(group);
    self
  }

  pub fn group(&self) -> Option<&Vec<String>> {
    self.group.as_ref()
  }

  pub fn reset_group(&mut self) {
    self.group = None;
  }

  pub fn set_non_resource_attributes(&mut self, non_resource_attributes: ::models::V1beta1NonResourceAttributes) {
    self.non_resource_attributes = Some(non_resource_attributes);
  }

  pub fn with_non_resource_attributes(mut self, non_resource_attributes: ::models::V1beta1NonResourceAttributes) -> V1beta1SubjectAccessReviewSpec {
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

  pub fn with_resource_attributes(mut self, resource_attributes: ::models::V1beta1ResourceAttributes) -> V1beta1SubjectAccessReviewSpec {
    self.resource_attributes = Some(resource_attributes);
    self
  }

  pub fn resource_attributes(&self) -> Option<&::models::V1beta1ResourceAttributes> {
    self.resource_attributes.as_ref()
  }

  pub fn reset_resource_attributes(&mut self) {
    self.resource_attributes = None;
  }

  pub fn set_uid(&mut self, uid: String) {
    self.uid = Some(uid);
  }

  pub fn with_uid(mut self, uid: String) -> V1beta1SubjectAccessReviewSpec {
    self.uid = Some(uid);
    self
  }

  pub fn uid(&self) -> Option<&String> {
    self.uid.as_ref()
  }

  pub fn reset_uid(&mut self) {
    self.uid = None;
  }

  pub fn set_user(&mut self, user: String) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: String) -> V1beta1SubjectAccessReviewSpec {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&String> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



