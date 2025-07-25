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
pub struct PaginationPeriodPaginatedResponseApimodelTag {
    /// The list of items in the current result set
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::ApimodelPeriodTag>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::PaginationPeriodPaginationMeta>>,
}

impl PaginationPeriodPaginatedResponseApimodelTag {
    pub fn new() -> PaginationPeriodPaginatedResponseApimodelTag {
        PaginationPeriodPaginatedResponseApimodelTag {
            data: None,
            pagination: None,
        }
    }
}

