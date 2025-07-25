/*
 * Anytype API
 *
 * This API enables seamless interaction with Anytype's resources - spaces, objects, properties, types, templates, and beyond.
 *
 * The version of the OpenAPI document: 2025-05-20
 * Contact: support@anytype.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_property`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePropertyError {
    Status400(models::UtilPeriodValidationError),
    Status401(models::UtilPeriodUnauthorizedError),
    Status429(models::UtilPeriodRateLimitError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_property`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePropertyError {
    Status401(models::UtilPeriodUnauthorizedError),
    Status403(models::UtilPeriodForbiddenError),
    Status404(models::UtilPeriodNotFoundError),
    Status410(models::UtilPeriodGoneError),
    Status429(models::UtilPeriodRateLimitError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_property`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPropertyError {
    Status401(models::UtilPeriodUnauthorizedError),
    Status404(models::UtilPeriodNotFoundError),
    Status410(models::UtilPeriodGoneError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_properties`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPropertiesError {
    Status401(models::UtilPeriodUnauthorizedError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_property`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePropertyError {
    Status400(models::UtilPeriodValidationError),
    Status401(models::UtilPeriodUnauthorizedError),
    Status403(models::UtilPeriodForbiddenError),
    Status404(models::UtilPeriodNotFoundError),
    Status410(models::UtilPeriodGoneError),
    Status429(models::UtilPeriodRateLimitError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}


/// ⚠ Warning: Properties are experimental and may change in the next update. ⚠ Creates a new property in the specified space using a JSON payload. The creation process is subject to rate limiting. The payload must include property details such as the name and format. The endpoint then returns the full property data, ready for further interactions.
pub async fn create_property(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, apimodel_period_create_property_request: models::ApimodelPeriodCreatePropertyRequest) -> Result<models::ApimodelPeriodPropertyResponse, Error<CreatePropertyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_apimodel_period_create_property_request = apimodel_period_create_property_request;

    let uri_str = format!("{}/v1/spaces/{space_id}/properties", configuration.base_path, space_id=crate::apis::urlencode(p_space_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Anytype-Version", p_anytype_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_apimodel_period_create_property_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreatePropertyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// ⚠ Warning: Properties are experimental and may change in the next update. ⚠ This endpoint “deletes” a property by marking it as archived. The deletion process is performed safely and is subject to rate limiting. It returns the property’s details after it has been archived. Proper error handling is in place for situations such as when the property isn’t found or the deletion cannot be performed because of permission issues.
pub async fn delete_property(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, property_id: &str) -> Result<models::ApimodelPeriodPropertyResponse, Error<DeletePropertyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_property_id = property_id;

    let uri_str = format!("{}/v1/spaces/{space_id}/properties/{property_id}", configuration.base_path, space_id=crate::apis::urlencode(p_space_id), property_id=crate::apis::urlencode(p_property_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Anytype-Version", p_anytype_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeletePropertyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// ⚠ Warning: Properties are experimental and may change in the next update. ⚠ Fetches detailed information about one specific property by its ID. This includes the property’s unique identifier, name and format. This detailed view assists clients in showing property options to users and in guiding the user interface (such as displaying appropriate input fields or selection options).
pub async fn get_property(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, property_id: &str) -> Result<models::ApimodelPeriodPropertyResponse, Error<GetPropertyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_property_id = property_id;

    let uri_str = format!("{}/v1/spaces/{space_id}/properties/{property_id}", configuration.base_path, space_id=crate::apis::urlencode(p_space_id), property_id=crate::apis::urlencode(p_property_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Anytype-Version", p_anytype_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPropertyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// ⚠ Warning: Properties are experimental and may change in the next update. ⚠ Retrieves a paginated list of properties available within a specific space. Each property record includes its unique identifier, name and format. This information is essential for clients to understand the available properties for filtering or creating objects.
pub async fn list_properties(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, offset: Option<i32>, limit: Option<i32>) -> Result<models::PaginationPeriodPaginatedResponseApimodelProperty, Error<ListPropertiesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_offset = offset;
    let p_limit = limit;

    let uri_str = format!("{}/v1/spaces/{space_id}/properties", configuration.base_path, space_id=crate::apis::urlencode(p_space_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Anytype-Version", p_anytype_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PaginationPeriodPaginatedResponseApimodelProperty`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PaginationPeriodPaginatedResponseApimodelProperty`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListPropertiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// ⚠ Warning: Properties are experimental and may change in the next update. ⚠ This endpoint updates an existing property in the specified space using a JSON payload. The update process is subject to rate limiting. The payload must include the name to be updated. The endpoint then returns the full property data, ready for further interactions.
pub async fn update_property(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, property_id: &str, apimodel_period_update_property_request: models::ApimodelPeriodUpdatePropertyRequest) -> Result<models::ApimodelPeriodPropertyResponse, Error<UpdatePropertyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_property_id = property_id;
    let p_apimodel_period_update_property_request = apimodel_period_update_property_request;

    let uri_str = format!("{}/v1/spaces/{space_id}/properties/{property_id}", configuration.base_path, space_id=crate::apis::urlencode(p_space_id), property_id=crate::apis::urlencode(p_property_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Anytype-Version", p_anytype_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_apimodel_period_update_property_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodPropertyResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdatePropertyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

