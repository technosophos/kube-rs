/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1NodeSelectorRequirement : A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1NodeSelectorRequirement {
  /// The label key that the selector applies to.
  #[serde(rename = "key")]
  key: String,
  /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
  #[serde(rename = "operator")]
  operator: String,
  /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
  #[serde(rename = "values")]
  values: Option<Vec<String>>
}

impl V1NodeSelectorRequirement {
  /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
  pub fn new(key: String, operator: String) -> V1NodeSelectorRequirement {
    V1NodeSelectorRequirement {
      key: key,
      operator: operator,
      values: None
    }
  }

  pub fn set_key(&mut self, key: String) {
    self.key = key;
  }

  pub fn with_key(mut self, key: String) -> V1NodeSelectorRequirement {
    self.key = key;
    self
  }

  pub fn key(&self) -> &String {
    &self.key
  }


  pub fn set_operator(&mut self, operator: String) {
    self.operator = operator;
  }

  pub fn with_operator(mut self, operator: String) -> V1NodeSelectorRequirement {
    self.operator = operator;
    self
  }

  pub fn operator(&self) -> &String {
    &self.operator
  }


  pub fn set_values(&mut self, values: Vec<String>) {
    self.values = Some(values);
  }

  pub fn with_values(mut self, values: Vec<String>) -> V1NodeSelectorRequirement {
    self.values = Some(values);
    self
  }

  pub fn values(&self) -> Option<&Vec<String>> {
    self.values.as_ref()
  }

  pub fn reset_values(&mut self) {
    self.values = None;
  }

}



