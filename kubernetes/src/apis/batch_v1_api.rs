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

pub struct BatchV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> BatchV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> BatchV1ApiClient<C> {
        BatchV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait BatchV1Api {
    fn create_namespaced_job(&self, namespace: &str, body: ::models::V1Job, include_uninitialized: bool, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>>;
    fn delete_collection_namespaced_job(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1Status, Error = Error>>;
    fn delete_namespaced_job(&self, name: &str, namespace: &str, body: ::models::V1DeleteOptions, pretty: &str, dry_run: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::V1Status, Error = Error>>;
    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>>;
    fn list_job_for_all_namespaces(&self, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1JobList, Error = Error>>;
    fn list_namespaced_job(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1JobList, Error = Error>>;
    fn patch_namespaced_job(&self, name: &str, namespace: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>>;
    fn patch_namespaced_job_status(&self, name: &str, namespace: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>>;
    fn read_namespaced_job(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::V1Job, Error = Error>>;
    fn read_namespaced_job_status(&self, name: &str, namespace: &str, pretty: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>>;
    fn replace_namespaced_job(&self, name: &str, namespace: &str, body: ::models::V1Job, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>>;
    fn replace_namespaced_job_status(&self, name: &str, namespace: &str, body: ::models::V1Job, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>>;
}


impl<C: hyper::client::Connect>BatchV1Api for BatchV1ApiClient<C> {
    fn create_namespaced_job(&self, namespace: &str, body: ::models::V1Job, include_uninitialized: bool, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs{}", configuration.base_path, query, namespace=namespace);

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
                let parsed: Result<::models::V1Job, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_collection_namespaced_job(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1Status, Error = Error>> {
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
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs{}", configuration.base_path, query, namespace=namespace);

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

    fn delete_namespaced_job(&self, name: &str, namespace: &str, body: ::models::V1DeleteOptions, pretty: &str, dry_run: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::V1Status, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .append_pair("gracePeriodSeconds", &grace_period_seconds.to_string())
            .append_pair("orphanDependents", &orphan_dependents.to_string())
            .append_pair("propagationPolicy", &propagation_policy.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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

    fn get_api_resources(&self, ) -> Box<Future<Item = ::models::V1ApiResourceList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/batch/v1/", configuration.base_path);

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

    fn list_job_for_all_namespaces(&self, _continue: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, limit: i32, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1JobList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("continue", &_continue.to_string())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/jobs{}", configuration.base_path, query);

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
                let parsed: Result<::models::V1JobList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn list_namespaced_job(&self, namespace: &str, include_uninitialized: bool, pretty: &str, _continue: &str, field_selector: &str, label_selector: &str, limit: i32, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::V1JobList, Error = Error>> {
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
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs{}", configuration.base_path, query, namespace=namespace);

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
                let parsed: Result<::models::V1JobList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn patch_namespaced_job(&self, name: &str, namespace: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1Job, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn patch_namespaced_job_status(&self, name: &str, namespace: &str, body: ::serde_json::Value, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs/{name}/status{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1Job, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn read_namespaced_job(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::V1Job, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("exact", &exact.to_string())
            .append_pair("export", &export.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1Job, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn read_namespaced_job_status(&self, name: &str, namespace: &str, pretty: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs/{name}/status{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1Job, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn replace_namespaced_job(&self, name: &str, namespace: &str, body: ::models::V1Job, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1Job, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn replace_namespaced_job_status(&self, name: &str, namespace: &str, body: ::models::V1Job, pretty: &str, dry_run: &str) -> Box<Future<Item = ::models::V1Job, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new("?".to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("dryRun", &dry_run.to_string())
            .finish();
        let uri_str = format!("{}/apis/batch/v1/namespaces/{namespace}/jobs/{name}/status{}", configuration.base_path, query, name=name, namespace=namespace);

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
                let parsed: Result<::models::V1Job, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
