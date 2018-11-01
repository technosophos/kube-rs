/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1CustomResourceValidation : CustomResourceValidation is a list of validation methods for CustomResources.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1CustomResourceValidation {
  /// OpenAPIV3Schema is the OpenAPI v3 schema to be validated against.
  #[serde(rename = "openAPIV3Schema")]
  open_apiv3_schema: Option<::models::V1beta1JsonSchemaProps>
}

impl V1beta1CustomResourceValidation {
  /// CustomResourceValidation is a list of validation methods for CustomResources.
  pub fn new() -> V1beta1CustomResourceValidation {
    V1beta1CustomResourceValidation {
      open_apiv3_schema: None
    }
  }

  pub fn set_open_apiv3_schema(&mut self, open_apiv3_schema: ::models::V1beta1JsonSchemaProps) {
    self.open_apiv3_schema = Some(open_apiv3_schema);
  }

  pub fn with_open_apiv3_schema(mut self, open_apiv3_schema: ::models::V1beta1JsonSchemaProps) -> V1beta1CustomResourceValidation {
    self.open_apiv3_schema = Some(open_apiv3_schema);
    self
  }

  pub fn open_apiv3_schema(&self) -> Option<&::models::V1beta1JsonSchemaProps> {
    self.open_apiv3_schema.as_ref()
  }

  pub fn reset_open_apiv3_schema(&mut self) {
    self.open_apiv3_schema = None;
  }

}



