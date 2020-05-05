// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: MIT

use macro_macro::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

macro_template!(serde_service_model = {
  #[derive(Default, Debug, Clone, Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[serde(deny_unknown_fields)]
  struct __ {
    #[serde(flatten)] #[serde(skip_serializing_if = "DocumentationFilter::skip_documentation")] __: Documentation,
    #[serde(flatten)] #[serde(skip_serializing_if = "DocumentationFilter::skip_documentation")] __: DocumentationUrl,
    #[serde(skip_serializing_if = "Option::is_none")] __: Option<!>,
    // #[serde(flatten)] extra: HashMap<String, Value>,
  }
});

pub static SER_SKIP_DOCUMENTATION: AtomicBool = AtomicBool::new(false);

#[macro_macro(serde_service_model)]
pub struct Service {
    pub version: Option<String>,
    pub documentation: Documentation,
    pub metadata: Option<Metadata>,
    pub operations: Option<HashMap<String, Operation>>,
    pub shapes: Option<HashMap<String, Shape>>,
    //TODO: allow extras fields extra: HashMap<String, Value>,
}

#[macro_macro(serde_service_model)]
pub struct Documentation {
    pub documentation: Option<String>,
}

#[macro_macro(serde_service_model)]
pub struct DocumentationUrl {
    pub documentation_url: Option<String>,
}

pub trait DocumentationFilter {
    fn skip_documentation(_item : &Self) -> bool {
        SER_SKIP_DOCUMENTATION.load(Ordering::Relaxed)
    }
}

impl DocumentationFilter for Documentation {}

impl DocumentationFilter for DocumentationUrl {}

#[macro_macro(serde_service_model)]
pub struct Metadata {
    pub api_version: Option<String>,
    pub json_version : Option<String>,
    pub endpoint_prefix: Option<String>,
    pub global_endpoint: Option<String>,
    pub protocol: Option<String>,
    pub service_abbreviation: Option<String>,
    pub service_full_name: Option<String>,
    pub service_id: Option<String>,
    pub checksum_format : Option<String>,
    pub timestamp_format : Option<String>,
    pub signature_version: Option<String>,
    pub signing_name : Option<String>,
    pub target_prefix : Option<String>,
    pub xml_namespace: Option<String>,
    pub uid: Option<String>,
}

#[macro_macro(serde_service_model)]
pub struct HttpOperation {
    pub method : String,
    pub request_uri : String,
    pub response_code : Option<i32>,
}

#[macro_macro(serde_service_model)]
pub struct Message {
    pub shape : String,
    pub location_name: Option<String>,
    pub xml_namespace: Option<XmlNamespace>,
    pub result_wrapper : Option<String>,
    pub documentation : Documentation,
}

#[macro_macro(serde_service_model)]
pub struct EndpointDiscovery {}

#[macro_macro(serde_service_model)]
pub struct Operation {
    pub name : String,
    pub alias : Option<String>,
    pub deprecated : Option<bool>,
    pub endpointdiscovery: Option<EndpointDiscovery>,
    pub endpointoperation: Option<bool>,
    pub idempotent: Option<bool>,
    pub http : Option<HttpOperation>,
    pub input : Option<Message>,
    pub output : Option<Message>,
    pub errors : Option<Vec<ShapeRef>>,
    pub documentation : Documentation,
    pub documentation_url : DocumentationUrl,
}

#[macro_macro(serde_service_model)]
pub struct XmlNamespace {
    pub prefix: Option<String>,
    pub uri: String,
}

#[macro_macro(serde_service_model)]
pub struct ShapeRef {
    pub shape : String,
    pub r#box: Option<bool>,
    pub deprecated: Option<bool>,
    pub eventpayload: Option<bool>,
    pub flattened : Option<bool>,
    pub idempotency_token: Option<bool>,
    pub streaming : Option<bool>,
    pub location: Option<String>,
    pub location_name: Option<String>,
    pub query_name : Option<String>,
    pub xml_attribute: Option<bool>,
    pub xml_namespace: Option<XmlNamespace>,
    pub documentation : Documentation,
}

#[macro_macro(serde_service_model)]
pub struct Error {
    pub code: Option<String>,
    pub http_status_code : i32,
    pub sender_fault : Option<bool>,
}

#[macro_macro(serde_service_model)]
pub struct Shape {
    pub r#type : String,
    pub r#box: Option<bool>,
    pub event : Option<bool>,
    pub eventstream : Option<bool>,
    pub flattened : Option<bool>,
    pub sensitive : Option<bool>,
    pub wrapper : Option<bool>,
    pub location_name: Option<String>,
    pub xml_namespace: Option<XmlNamespace>,

    // shape meta
    pub deprecated : Option<bool>,
    pub deprecated_message : Option<String>,

    // number type
    pub min : Option<i32>,
    pub max : Option<i32>,

    // string type values
    pub r#enum : Option<Vec<String>>,
    pub pattern : Option<String>,

    // timestamp type
    pub timestamp_format : Option<String>,

    // map type
    pub key : Option<ShapeRef>,
    pub value : Option<ShapeRef>,

    // error type
    pub error : Option<Error>,
    pub exception : Option<bool>,
    pub fault : Option<bool>,

    pub required : Option<Vec<String>>,
    pub member : Option<ShapeRef>,
    pub members : Option<HashMap<String, ShapeRef>>,
    pub payload: Option<String>,
    pub documentation : Documentation,
}
