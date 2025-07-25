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


/// struct for typed errors of method [`create_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTypeError {
    Status400(models::UtilPeriodValidationError),
    Status401(models::UtilPeriodUnauthorizedError),
    Status429(models::UtilPeriodRateLimitError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTypeError {
    Status401(models::UtilPeriodUnauthorizedError),
    Status403(models::UtilPeriodForbiddenError),
    Status404(models::UtilPeriodNotFoundError),
    Status410(models::UtilPeriodGoneError),
    Status429(models::UtilPeriodRateLimitError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTypeError {
    Status401(models::UtilPeriodUnauthorizedError),
    Status404(models::UtilPeriodNotFoundError),
    Status410(models::UtilPeriodGoneError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_types`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTypesError {
    Status401(models::UtilPeriodUnauthorizedError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTypeError {
    Status400(models::UtilPeriodValidationError),
    Status401(models::UtilPeriodUnauthorizedError),
    Status404(models::UtilPeriodNotFoundError),
    Status410(models::UtilPeriodGoneError),
    Status429(models::UtilPeriodRateLimitError),
    Status500(models::UtilPeriodServerError),
    UnknownValue(serde_json::Value),
}


/// Creates a new type in the specified space using a JSON payload. The creation process is subject to rate limiting. The payload must include type details such as the name, icon, and layout. The endpoint then returns the full type data, ready to be used for creating objects.
pub async fn create_type(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, apimodel_period_create_type_request: models::ApimodelPeriodCreateTypeRequest) -> Result<models::ApimodelPeriodTypeResponse, Error<CreateTypeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_apimodel_period_create_type_request = apimodel_period_create_type_request;

    let uri_str = format!("{}/v1/spaces/{space_id}/types", configuration.base_path, space_id=crate::apis::urlencode(p_space_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Anytype-Version", p_anytype_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_apimodel_period_create_type_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// This endpoint “deletes” an type by marking it as archived. The deletion process is performed safely and is subject to rate limiting. It returns the type’s details after it has been archived. Proper error handling is in place for situations such as when the type isn’t found or the deletion cannot be performed because of permission issues.
pub async fn delete_type(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, type_id: &str) -> Result<models::ApimodelPeriodTypeResponse, Error<DeleteTypeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_type_id = type_id;

    let uri_str = format!("{}/v1/spaces/{space_id}/types/{type_id}", configuration.base_path, space_id=crate::apis::urlencode(p_space_id), type_id=crate::apis::urlencode(p_type_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Fetches detailed information about one specific type by its ID. This includes the type’s unique key, name, icon, and layout. This detailed view assists clients in understanding the expected structure and style for objects of that type and in guiding the user interface (such as displaying appropriate icons or layout hints).
pub async fn get_type(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, type_id: &str) -> Result<models::ApimodelPeriodTypeResponse, Error<GetTypeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_type_id = type_id;

    let uri_str = format!("{}/v1/spaces/{space_id}/types/{type_id}", configuration.base_path, space_id=crate::apis::urlencode(p_space_id), type_id=crate::apis::urlencode(p_type_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// This endpoint retrieves a paginated list of types (e.g. 'Page', 'Note', 'Task') available within the specified space. Each type’s record includes its unique identifier, type key, display name, icon, and layout. While a type's id is truly unique, a type's key can be the same across spaces for known types, e.g. 'page' for 'Page'. Clients use this information when offering choices for object creation or for filtering objects by type through search.
pub async fn list_types(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, offset: Option<i32>, limit: Option<i32>) -> Result<models::PaginationPeriodPaginatedResponseApimodelType, Error<ListTypesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_offset = offset;
    let p_limit = limit;

    let uri_str = format!("{}/v1/spaces/{space_id}/types", configuration.base_path, space_id=crate::apis::urlencode(p_space_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PaginationPeriodPaginatedResponseApimodelType`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PaginationPeriodPaginatedResponseApimodelType`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListTypesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// This endpoint updates an existing type in the specified space using a JSON payload. The update process is subject to rate limiting. The payload must include the name and properties to be updated. The endpoint then returns the full type data, ready for further interactions.
pub async fn update_type(configuration: &configuration::Configuration, anytype_version: &str, space_id: &str, type_id: &str, apimodel_period_update_type_request: models::ApimodelPeriodUpdateTypeRequest) -> Result<models::ApimodelPeriodTypeResponse, Error<UpdateTypeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_anytype_version = anytype_version;
    let p_space_id = space_id;
    let p_type_id = type_id;
    let p_apimodel_period_update_type_request = apimodel_period_update_type_request;

    let uri_str = format!("{}/v1/spaces/{space_id}/types/{type_id}", configuration.base_path, space_id=crate::apis::urlencode(p_space_id), type_id=crate::apis::urlencode(p_type_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Anytype-Version", p_anytype_version.to_string());
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_apimodel_period_update_type_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApimodelPeriodTypeResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

