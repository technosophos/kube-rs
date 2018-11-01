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

pub struct AuthorizationV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AuthorizationV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthorizationV1ApiClient<C> {
        AuthorizationV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthorizationV1Api {
    fn create_namespaced_local_subject_access_review(&self, namespace: &str, body: ::models::V1LocalSubjectAccessReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1LocalSubjectAccessReview, Error = Error>>;
    fn create_self_subject_access_review(&self, body: ::models::V1SelfSubjectAccessReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1SelfSubjectAccessReview, Error = Error>>;
    fn create_self_subject_rules_review(&self, body: ::models::V1SelfSubjectRulesReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1SelfSubjectRulesReview, Error = Error>>;
    fn create_subject_access_review(&self, body: ::models::V1SubjectAccessReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1SubjectAccessReview, Error = Error>>;
    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>>;
}


impl<C: hyper::client::Connect>AuthorizationV1Api for AuthorizationV1ApiClient<C> {
    fn create_namespaced_local_subject_access_review(&self, namespace: &str, body: ::models::V1LocalSubjectAccessReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1LocalSubjectAccessReview, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("dryRun", &dry_run.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/authorization.k8s.io/v1/namespaces/{namespace}/localsubjectaccessreviews{}", configuration.base_path, query, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1LocalSubjectAccessReview, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn create_self_subject_access_review(&self, body: ::models::V1SelfSubjectAccessReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1SelfSubjectAccessReview, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("dryRun", &dry_run.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/authorization.k8s.io/v1/selfsubjectaccessreviews{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1SelfSubjectAccessReview, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn create_self_subject_rules_review(&self, body: ::models::V1SelfSubjectRulesReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1SelfSubjectRulesReview, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("dryRun", &dry_run.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/authorization.k8s.io/v1/selfsubjectrulesreviews{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1SelfSubjectRulesReview, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn create_subject_access_review(&self, body: ::models::V1SubjectAccessReview, dry_run: &str, include_uninitialized: bool, pretty: &str) -> Box<Future<Item = ::models::V1SubjectAccessReview, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("dryRun", &dry_run.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/authorization.k8s.io/v1/subjectaccessreviews{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::V1SubjectAccessReview, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/authorization.k8s.io/v1/", configuration.base_path);

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
                let parsed: Result<::models::V1ApiResourceList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
