/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.12.3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct RbacAuthorizationApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> RbacAuthorizationApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> RbacAuthorizationApiClient<C> {
        RbacAuthorizationApiClient {
            configuration: configuration,
        }
    }
}

pub trait RbacAuthorizationApi {
    fn get_api_group(&self, ) -> Box<Future<Item = ::models::V1ApiGroup, Error = Error>>;
}


impl<C: hyper::client::Connect>RbacAuthorizationApi for RbacAuthorizationApiClient<C> {
    fn get_api_group(&self, ) -> Box<Future<Item = ::models::V1ApiGroup, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/rbac.authorization.k8s.io/", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiGroup, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
