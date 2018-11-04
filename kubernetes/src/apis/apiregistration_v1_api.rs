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

pub struct ApiregistrationV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ApiregistrationV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ApiregistrationV1ApiClient<C> {
        ApiregistrationV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait ApiregistrationV1Api {
    fn create_api_service(&self, body: ::models::V1ApiService, include_uninitialized: bool, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>>;
    fn delete_api_service(&self, name: &str, body: ::models::V1DeleteOptions, pretty: &str, dry_run: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::V1Status, Error = Error>>;
    fn delete_collection_api_service(&self, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1Status, Error = Error>>;
    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>>;
    fn list_api_service(&self, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1ApiServiceList, Error = Error>>;
    fn patch_api_service(&self, name: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>>;
    fn patch_api_service_status(&self, name: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>>;
    fn read_api_service(&self, name: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::V1ApiService, Error = Error>>;
    fn read_api_service_status(&self, name: &str, pretty: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>>;
    fn replace_api_service(&self, name: &str, body: ::models::V1ApiService, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>>;
    fn replace_api_service_status(&self, name: &str, body: ::models::V1ApiService, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>>;
}


impl<C: hyper::client::Connect>ApiregistrationV1Api for ApiregistrationV1ApiClient<C> {
    fn create_api_service(&self, body: ::models::V1ApiService, include_uninitialized: bool, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiService, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_api_service(&self, name: &str, body: ::models::V1DeleteOptions, pretty: &str, dry_run: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::V1Status, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .append_pair("gracePeriodSeconds", &grace_period_seconds.to_string())
            .append_pair("orphanDependents", &orphan_dependents.to_string())
            .append_pair("propagationPolicy", &propagation_policy.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices/{name}{}", configuration.base_path, query, name=name);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1Status, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_collection_api_service(&self, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1Status, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("continue", &_continue.to_string())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1Status, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiResourceList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn list_api_service(&self, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1ApiServiceList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("continue", &_continue.to_string())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiServiceList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn patch_api_service(&self, name: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices/{name}{}", configuration.base_path, query, name=name);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiService, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn patch_api_service_status(&self, name: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices/{name}/status{}", configuration.base_path, query, name=name);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiService, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn read_api_service(&self, name: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::V1ApiService, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("exact", &exact.to_string())
            .append_pair("export", &export.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices/{name}{}", configuration.base_path, query, name=name);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiService, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn read_api_service_status(&self, name: &str, pretty: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices/{name}/status{}", configuration.base_path, query, name=name);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiService, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn replace_api_service(&self, name: &str, body: ::models::V1ApiService, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices/{name}{}", configuration.base_path, query, name=name);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiService, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn replace_api_service_status(&self, name: &str, body: ::models::V1ApiService, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1ApiService, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/apiregistration.k8s.io/v1/apiservices/{name}/status{}", configuration.base_path, query, name=name);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
                let mut req = hyper::Request::new(method, uri.unwrap());
        match (&configuration.bearer, &configuration.basic) {
            (Some(b), _) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            (_, Some(b)) => req.headers_mut().set(hyper::header::Authorization(b.clone())),
            _ => {}
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1ApiService, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
