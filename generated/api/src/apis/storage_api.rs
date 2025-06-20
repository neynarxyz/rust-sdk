/*
 * Farcaster API V2
 *
 * The Farcaster API allows you to interact with the Farcaster protocol. See the [Neynar docs](https://docs.neynar.com/reference) for more details.
 *
 * The version of the OpenAPI document: 2.46.0
 * Contact: team@neynar.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for passing parameters to the method [`buy_storage`]
#[derive(Clone, Debug)]
pub struct BuyStorageParams {
    pub buy_storage_req_body: models::BuyStorageReqBody,
}

/// struct for passing parameters to the method [`lookup_user_storage_allocations`]
#[derive(Clone, Debug)]
pub struct LookupUserStorageAllocationsParams {
    pub fid: i32,
}

/// struct for passing parameters to the method [`lookup_user_storage_usage`]
#[derive(Clone, Debug)]
pub struct LookupUserStorageUsageParams {
    pub fid: i32,
}

/// struct for typed errors of method [`buy_storage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuyStorageError {
    Status400(models::ZodError),
    Status409(models::ConflictErrorRes),
    Status500(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lookup_user_storage_allocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LookupUserStorageAllocationsError {
    Status400(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lookup_user_storage_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LookupUserStorageUsageError {
    Status400(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// This api will help you rent units of storage for an year for a specific FID. A storage unit lets you store 5000 casts, 2500 reactions and 2500 links.
pub async fn buy_storage(
    configuration: &configuration::Configuration,
    params: BuyStorageParams,
) -> Result<models::StorageAllocationsResponse, Error<BuyStorageError>> {
    let uri_str = format!("{}/farcaster/storage/buy", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
    };
    req_builder = req_builder.header(reqwest::header::CONTENT_TYPE, "application/json");
    req_builder = req_builder.json(&params.buy_storage_req_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StorageAllocationsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StorageAllocationsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BuyStorageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Fetches storage allocations for a given user
pub async fn lookup_user_storage_allocations(
    configuration: &configuration::Configuration,
    params: LookupUserStorageAllocationsParams,
) -> Result<models::StorageAllocationsResponse, Error<LookupUserStorageAllocationsError>> {
    let uri_str = format!("{}/farcaster/storage/allocations", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("fid", &params.fid.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StorageAllocationsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StorageAllocationsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LookupUserStorageAllocationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Fetches storage usage for a given user
pub async fn lookup_user_storage_usage(
    configuration: &configuration::Configuration,
    params: LookupUserStorageUsageParams,
) -> Result<models::StorageUsageResponse, Error<LookupUserStorageUsageError>> {
    let uri_str = format!("{}/farcaster/storage/usage", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("fid", &params.fid.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-key", value);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StorageUsageResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StorageUsageResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LookupUserStorageUsageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
