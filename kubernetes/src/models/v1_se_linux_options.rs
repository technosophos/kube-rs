/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1SeLinuxOptions : SELinuxOptions are the labels to be applied to the container

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1SeLinuxOptions {
  /// Level is SELinux level label that applies to the container.
  #[serde(rename = "level")]
  level: Option<String>,
  /// Role is a SELinux role label that applies to the container.
  #[serde(rename = "role")]
  role: Option<String>,
  /// Type is a SELinux type label that applies to the container.
  #[serde(rename = "type")]
  _type: Option<String>,
  /// User is a SELinux user label that applies to the container.
  #[serde(rename = "user")]
  user: Option<String>
}

impl V1SeLinuxOptions {
  /// SELinuxOptions are the labels to be applied to the container
  pub fn new() -> V1SeLinuxOptions {
    V1SeLinuxOptions {
      level: None,
      role: None,
      _type: None,
      user: None
    }
  }

  pub fn set_level(&mut self, level: String) {
    self.level = Some(level);
  }

  pub fn with_level(mut self, level: String) -> V1SeLinuxOptions {
    self.level = Some(level);
    self
  }

  pub fn level(&self) -> Option<&String> {
    self.level.as_ref()
  }

  pub fn reset_level(&mut self) {
    self.level = None;
  }

  pub fn set_role(&mut self, role: String) {
    self.role = Some(role);
  }

  pub fn with_role(mut self, role: String) -> V1SeLinuxOptions {
    self.role = Some(role);
    self
  }

  pub fn role(&self) -> Option<&String> {
    self.role.as_ref()
  }

  pub fn reset_role(&mut self) {
    self.role = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> V1SeLinuxOptions {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_user(&mut self, user: String) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: String) -> V1SeLinuxOptions {
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



