/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1GroupVersionForDiscovery : GroupVersion contains the \"group/version\" and \"version\" string of a version. It is made a struct to keep extensibility.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1GroupVersionForDiscovery {
  /// groupVersion specifies the API group and version in the form \"group/version\"
  #[serde(rename = "groupVersion")]
  group_version: String,
  /// version specifies the version in the form of \"version\". This is to save the clients the trouble of splitting the GroupVersion.
  #[serde(rename = "version")]
  version: String
}

impl V1GroupVersionForDiscovery {
  /// GroupVersion contains the \"group/version\" and \"version\" string of a version. It is made a struct to keep extensibility.
  pub fn new(group_version: String, version: String) -> V1GroupVersionForDiscovery {
    V1GroupVersionForDiscovery {
      group_version: group_version,
      version: version
    }
  }

  pub fn set_group_version(&mut self, group_version: String) {
    self.group_version = group_version;
  }

  pub fn with_group_version(mut self, group_version: String) -> V1GroupVersionForDiscovery {
    self.group_version = group_version;
    self
  }

  pub fn group_version(&self) -> &String {
    &self.group_version
  }


  pub fn set_version(&mut self, version: String) {
    self.version = version;
  }

  pub fn with_version(mut self, version: String) -> V1GroupVersionForDiscovery {
    self.version = version;
    self
  }

  pub fn version(&self) -> &String {
    &self.version
  }


}



