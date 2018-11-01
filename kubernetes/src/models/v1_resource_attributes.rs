/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ResourceAttributes : ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ResourceAttributes {
  /// Group is the API Group of the Resource.  \"*\" means all.
  #[serde(rename = "group")]
  group: Option<String>,
  /// Name is the name of the resource being requested for a \"get\" or deleted for a \"delete\". \"\" (empty) means all.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces \"\" (empty) is defaulted for LocalSubjectAccessReviews \"\" (empty) is empty for cluster-scoped resources \"\" (empty) means \"all\" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
  #[serde(rename = "namespace")]
  namespace: Option<String>,
  /// Resource is one of the existing resource types.  \"*\" means all.
  #[serde(rename = "resource")]
  resource: Option<String>,
  /// Subresource is one of the existing resource types.  \"\" means none.
  #[serde(rename = "subresource")]
  subresource: Option<String>,
  /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.
  #[serde(rename = "verb")]
  verb: Option<String>,
  /// Version is the API Version of the Resource.  \"*\" means all.
  #[serde(rename = "version")]
  version: Option<String>
}

impl V1ResourceAttributes {
  /// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
  pub fn new() -> V1ResourceAttributes {
    V1ResourceAttributes {
      group: None,
      name: None,
      namespace: None,
      resource: None,
      subresource: None,
      verb: None,
      version: None
    }
  }

  pub fn set_group(&mut self, group: String) {
    self.group = Some(group);
  }

  pub fn with_group(mut self, group: String) -> V1ResourceAttributes {
    self.group = Some(group);
    self
  }

  pub fn group(&self) -> Option<&String> {
    self.group.as_ref()
  }

  pub fn reset_group(&mut self) {
    self.group = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> V1ResourceAttributes {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_namespace(&mut self, namespace: String) {
    self.namespace = Some(namespace);
  }

  pub fn with_namespace(mut self, namespace: String) -> V1ResourceAttributes {
    self.namespace = Some(namespace);
    self
  }

  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  pub fn reset_namespace(&mut self) {
    self.namespace = None;
  }

  pub fn set_resource(&mut self, resource: String) {
    self.resource = Some(resource);
  }

  pub fn with_resource(mut self, resource: String) -> V1ResourceAttributes {
    self.resource = Some(resource);
    self
  }

  pub fn resource(&self) -> Option<&String> {
    self.resource.as_ref()
  }

  pub fn reset_resource(&mut self) {
    self.resource = None;
  }

  pub fn set_subresource(&mut self, subresource: String) {
    self.subresource = Some(subresource);
  }

  pub fn with_subresource(mut self, subresource: String) -> V1ResourceAttributes {
    self.subresource = Some(subresource);
    self
  }

  pub fn subresource(&self) -> Option<&String> {
    self.subresource.as_ref()
  }

  pub fn reset_subresource(&mut self) {
    self.subresource = None;
  }

  pub fn set_verb(&mut self, verb: String) {
    self.verb = Some(verb);
  }

  pub fn with_verb(mut self, verb: String) -> V1ResourceAttributes {
    self.verb = Some(verb);
    self
  }

  pub fn verb(&self) -> Option<&String> {
    self.verb.as_ref()
  }

  pub fn reset_verb(&mut self) {
    self.verb = None;
  }

  pub fn set_version(&mut self, version: String) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: String) -> V1ResourceAttributes {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&String> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

}



