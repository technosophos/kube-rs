/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.17
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

pub struct NetworkingV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NetworkingV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NetworkingV1ApiClient<C> {
        NetworkingV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait NetworkingV1Api {
    fn create_namespaced_network_policy(&self, namespace: &str, body: ::models::V1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>>;
    fn delete_collection_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1Status, Error = Error>>;
    fn delete_namespaced_network_policy(&self, name: &str, namespace: &str, body: ::models::V1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::V1Status, Error = Error>>;
    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>>;
    fn list_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1NetworkPolicyList, Error = Error>>;
    fn list_network_policy_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1NetworkPolicyList, Error = Error>>;
    fn patch_namespaced_network_policy(&self, name: &str, namespace: &str, body: ::serde_json::Value, pretty: &str) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>>;
    fn read_namespaced_network_policy(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>>;
    fn replace_namespaced_network_policy(&self, name: &str, namespace: &str, body: ::models::V1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>>;
}


impl<C: hyper::client::Connect>NetworkingV1Api for NetworkingV1ApiClient<C> {
    fn create_namespaced_network_policy(&self, namespace: &str, body: ::models::V1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies{}", configuration.base_path, query, namespace=namespace);

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
                let parsed: Result<::models::V1NetworkPolicy, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_collection_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1Status, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies{}", configuration.base_path, query, namespace=namespace);

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
                let parsed: Result<::models::V1Status, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_namespaced_network_policy(&self, name: &str, namespace: &str, body: ::models::V1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::V1Status, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("gracePeriodSeconds", &grace_period_seconds.to_string())
            .append_pair("orphanDependents", &orphan_dependents.to_string())
            .append_pair("propagationPolicy", &propagation_policy.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1Status, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/networking.k8s.io/v1/", configuration.base_path);

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

    fn list_namespaced_network_policy(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1NetworkPolicyList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies{}", configuration.base_path, query, namespace=namespace);

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
                let parsed: Result<::models::V1NetworkPolicyList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn list_network_policy_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1NetworkPolicyList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/networkpolicies{}", configuration.base_path, query);

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
                let parsed: Result<::models::V1NetworkPolicyList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn patch_namespaced_network_policy(&self, name: &str, namespace: &str, body: ::serde_json::Value, pretty: &str) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1NetworkPolicy, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn read_namespaced_network_policy(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("exact", &exact.to_string())
            .append_pair("export", &export.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1NetworkPolicy, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn replace_namespaced_network_policy(&self, name: &str, namespace: &str, body: ::models::V1NetworkPolicy, pretty: &str) -> Box<Future<Item = ::models::V1NetworkPolicy, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/networking.k8s.io/v1/namespaces/{namespace}/networkpolicies/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1NetworkPolicy, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
