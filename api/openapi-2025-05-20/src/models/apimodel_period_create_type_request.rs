/*
 * Anytype API
 *
 * This API enables seamless interaction with Anytype's resources - spaces, objects, properties, types, templates, and beyond.
 *
 * The version of the OpenAPI document: 2025-05-20
 * Contact: support@anytype.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApimodelPeriodCreateTypeRequest {
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<models::ApimodelPeriodIcon>>,
    /// The key of the type; should always be snake_case, otherwise it will be converted to snake_case
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "layout")]
    pub layout: models::ApimodelPeriodTypeLayout,
    /// The name of the type
    #[serde(rename = "name")]
    pub name: String,
    /// The plural name of the type
    #[serde(rename = "plural_name")]
    pub plural_name: String,
    /// ⚠ Warning: Properties are experimental and may change in the next update. ⚠ The properties linked to the type
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<models::ApimodelPeriodPropertyLink>>,
}

impl ApimodelPeriodCreateTypeRequest {
    pub fn new(layout: models::ApimodelPeriodTypeLayout, name: String, plural_name: String) -> ApimodelPeriodCreateTypeRequest {
        ApimodelPeriodCreateTypeRequest {
            icon: None,
            key: None,
            layout,
            name,
            plural_name,
            properties: None,
        }
    }
}

