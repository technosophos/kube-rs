/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use hyper;

pub struct Configuration<C: hyper::client::Connect> {
  pub base_path: String,
  pub client: hyper::client::Client<C>,
  pub bearer: Option<hyper::header::Bearer>,
  pub basic: Option<hyper::header::Basic>,
}

impl<C: hyper::client::Connect> Configuration<C> {
  pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
    Configuration {
      base_path: "https://localhost".to_owned(),
      client: client,
      bearer: None,
      basic: None,
    }
  }
}
