/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1SubjectAccessReviewStatus : SubjectAccessReviewStatus

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1SubjectAccessReviewStatus {
  /// Allowed is required.  True if the action would be allowed, false otherwise.
  #[serde(rename = "allowed")]
  allowed: bool,
  /// EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
  #[serde(rename = "evaluationError")]
  evaluation_error: Option<String>,
  /// Reason is optional.  It indicates why a request was allowed or denied.
  #[serde(rename = "reason")]
  reason: Option<String>
}

impl V1beta1SubjectAccessReviewStatus {
  /// SubjectAccessReviewStatus
  pub fn new(allowed: bool) -> V1beta1SubjectAccessReviewStatus {
    V1beta1SubjectAccessReviewStatus {
      allowed: allowed,
      evaluation_error: None,
      reason: None
    }
  }

  pub fn set_allowed(&mut self, allowed: bool) {
    self.allowed = allowed;
  }

  pub fn with_allowed(mut self, allowed: bool) -> V1beta1SubjectAccessReviewStatus {
    self.allowed = allowed;
    self
  }

  pub fn allowed(&self) -> &bool {
    &self.allowed
  }


  pub fn set_evaluation_error(&mut self, evaluation_error: String) {
    self.evaluation_error = Some(evaluation_error);
  }

  pub fn with_evaluation_error(mut self, evaluation_error: String) -> V1beta1SubjectAccessReviewStatus {
    self.evaluation_error = Some(evaluation_error);
    self
  }

  pub fn evaluation_error(&self) -> Option<&String> {
    self.evaluation_error.as_ref()
  }

  pub fn reset_evaluation_error(&mut self) {
    self.evaluation_error = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> V1beta1SubjectAccessReviewStatus {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

}



