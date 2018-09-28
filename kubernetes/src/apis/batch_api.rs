/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
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

pub struct BatchApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> BatchApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> BatchApiClient<C> {
        BatchApiClient {
            configuration: configuration,
        }
    }
}

pub trait BatchApi {
    fn get_batch_api_group(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error = Error>>;
}


impl<C: hyper::client::Connect>BatchApi for BatchApiClient<C> {
    fn get_batch_api_group(&self, ) -> Box<Future<Item = ::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/batch/", configuration.base_path);

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
                let parsed: Result<::models::IoK8sApimachineryPkgApisMetaV1ApiGroup, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
