/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1SelfSubjectRulesReviewSpec {
  /// Namespace to evaluate rules for. Required.
  #[serde(rename = "namespace")]
  namespace: Option<String>
}

impl V1SelfSubjectRulesReviewSpec {
  pub fn new() -> V1SelfSubjectRulesReviewSpec {
    V1SelfSubjectRulesReviewSpec {
      namespace: None
    }
  }

  pub fn set_namespace(&mut self, namespace: String) {
    self.namespace = Some(namespace);
  }

  pub fn with_namespace(mut self, namespace: String) -> V1SelfSubjectRulesReviewSpec {
    self.namespace = Some(namespace);
    self
  }

  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  pub fn reset_namespace(&mut self) {
    self.namespace = None;
  }

}



