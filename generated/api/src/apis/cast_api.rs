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

/// struct for passing parameters to the method [`delete_cast`]
#[derive(Clone, Debug)]
pub struct DeleteCastParams {
    pub delete_cast_req_body: models::DeleteCastReqBody,
}

/// struct for passing parameters to the method [`fetch_bulk_casts`]
#[derive(Clone, Debug)]
pub struct FetchBulkCastsParams {
    /// Hashes of the cast to be retrived (Comma separated, no spaces)
    pub casts: String,
    /// adds viewer_context to cast object to show whether viewer has liked or recasted the cast.
    pub viewer_fid: Option<i32>,
    /// Optional parameter to sort the casts based on different criteria
    pub sort_type: Option<String>,
    /// Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details.
    pub x_neynar_experimental: Option<bool>,
}

/// struct for passing parameters to the method [`fetch_composer_actions`]
#[derive(Clone, Debug)]
pub struct FetchComposerActionsParams {
    /// Type of list to fetch.
    pub list: models::CastComposerType,
    /// Number of results to fetch
    pub limit: Option<i32>,
    /// Pagination cursor.
    pub cursor: Option<String>,
}

/// struct for passing parameters to the method [`fetch_embedded_url_metadata`]
#[derive(Clone, Debug)]
pub struct FetchEmbeddedUrlMetadataParams {
    /// URL to crawl metadata of
    pub url: String,
}

/// struct for passing parameters to the method [`lookup_cast_by_hash_or_warpcast_url`]
#[derive(Clone, Debug)]
pub struct LookupCastByHashOrWarpcastUrlParams {
    /// Cast identifier (Its either a url or a hash)
    pub identifier: String,
    pub r#type: models::CastParamType,
    /// adds viewer_context to cast object to show whether viewer has liked or recasted the cast.
    pub viewer_fid: Option<i32>,
    /// Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details.
    pub x_neynar_experimental: Option<bool>,
}

/// struct for passing parameters to the method [`lookup_cast_conversation`]
#[derive(Clone, Debug)]
pub struct LookupCastConversationParams {
    /// Cast identifier (Its either a url or a hash)
    pub identifier: String,
    pub r#type: models::CastParamType,
    /// The depth of replies in the conversation that will be returned (default 2)
    pub reply_depth: Option<i32>,
    /// Include all parent casts in chronological order
    pub include_chronological_parent_casts: Option<bool>,
    /// Providing this will return a conversation that respects this user's mutes and blocks and includes `viewer_context`.
    pub viewer_fid: Option<i32>,
    /// Sort type for the ordering of descendants. Default is `chron`
    pub sort_type: Option<models::CastConversationSortType>,
    /// Show conversation above or below the fold. Lower quality responses are hidden below the fold. Not passing in a value shows the full conversation without any folding.
    pub fold: Option<String>,
    /// Number of results to fetch
    pub limit: Option<i32>,
    /// Pagination cursor.
    pub cursor: Option<String>,
    /// Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details.
    pub x_neynar_experimental: Option<bool>,
}

/// struct for passing parameters to the method [`publish_cast`]
#[derive(Clone, Debug)]
pub struct PublishCastParams {
    pub post_cast_req_body: models::PostCastReqBody,
}

/// struct for passing parameters to the method [`search_casts`]
#[derive(Clone, Debug)]
pub struct SearchCastsParams {
    /// Query string to search for casts. Supported operators:  | Operator  | Description                                                                                              | | --------- | -------------------------------------------------------------------------------------------------------- | | `+`       | Acts as the AND operator. This is the default operator between terms and can usually be omitted.         | | `\\|`      | Acts as the OR operator.                                                                                 | | `*`       | When used at the end of a term, signifies a prefix query.                                                  | | `\"`       | Wraps several terms into a phrase (for example, `\"star wars\"`).                                          | | `(`, `)`  | Wrap a clause for precedence (for example, `star + (wars \\| trek)`).                                     | | `~n`      | When used after a term (for example, `satr~3`), sets `fuzziness`. When used after a phrase, sets `slop`. | | `-`       | Negates the term.                                                                                        | | `before:` | Search for casts before a specific date. (e.g. `before:2025-04-20`)                                       | | `after:`  | Search for casts after a specific date. (e.g. `after:2025-04-20`)                                         |
    pub q: String,
    /// Choices are: - `literal` - Searches for the words in the query string (default) - `semantic` - Searches for the meaning of the query string - `hybrid` - Combines both literal and semantic results
    pub mode: Option<String>,
    /// Choices are: - `desc_chron` - All casts sorted by time (default) - `algorithmic` - Casts sorted by engagement and time
    pub sort_type: Option<models::SearchSortType>,
    /// Fid of the user whose casts you want to search
    pub author_fid: Option<i32>,
    /// Providing this will return search results that respects this user's mutes and blocks and includes `viewer_context`.
    pub viewer_fid: Option<i32>,
    /// Parent URL of the casts you want to search
    pub parent_url: Option<String>,
    /// Channel ID of the casts you want to search
    pub channel_id: Option<String>,
    /// When true, only returns search results from power badge users and users that the viewer follows (if viewer_fid is provided).
    pub priority_mode: Option<bool>,
    /// Number of results to fetch
    pub limit: Option<i32>,
    /// Pagination cursor
    pub cursor: Option<String>,
    /// Enables experimental features including filtering based on the Neynar score. See [docs](https://neynar.notion.site/Experimental-Features-1d2655195a8b80eb98b4d4ae7b76ae4a) for more details.
    pub x_neynar_experimental: Option<bool>,
}

/// struct for typed errors of method [`delete_cast`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCastError {
    Status400(models::ErrorRes),
    Status403(models::ErrorRes),
    Status404(models::ErrorRes),
    Status500(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_bulk_casts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchBulkCastsError {
    Status400(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_composer_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchComposerActionsError {
    Status400(models::ErrorRes),
    Status500(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_embedded_url_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchEmbeddedUrlMetadataError {
    Status400(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lookup_cast_by_hash_or_warpcast_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LookupCastByHashOrWarpcastUrlError {
    Status400(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lookup_cast_conversation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LookupCastConversationError {
    Status400(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`publish_cast`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishCastError {
    Status400(models::ErrorRes),
    Status403(models::ErrorRes),
    Status404(models::ErrorRes),
    Status500(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_casts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCastsError {
    Status400(models::ErrorRes),
    UnknownValue(serde_json::Value),
}

/// Delete an existing cast. \\ (In order to delete a cast `signer_uuid` must be approved)
pub async fn delete_cast(
    configuration: &configuration::Configuration,
    params: DeleteCastParams,
) -> Result<models::OperationResponse, Error<DeleteCastError>> {
    let uri_str = format!("{}/farcaster/cast", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

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
    req_builder = req_builder.json(&params.delete_cast_req_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OperationResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OperationResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteCastError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Fetch multiple casts using their respective hashes.
pub async fn fetch_bulk_casts(
    configuration: &configuration::Configuration,
    params: FetchBulkCastsParams,
) -> Result<models::CastsResponse, Error<FetchBulkCastsError>> {
    let uri_str = format!("{}/farcaster/casts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("casts", &params.casts.to_string())]);
    if let Some(ref param_value) = params.viewer_fid {
        req_builder = req_builder.query(&[("viewer_fid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sort_type {
        req_builder = req_builder.query(&[("sort_type", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_neynar_experimental {
        req_builder = req_builder.header("x-neynar-experimental", param_value.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CastsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CastsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchBulkCastsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Fetches all composer actions on Warpcast. You can filter by top or featured.
pub async fn fetch_composer_actions(
    configuration: &configuration::Configuration,
    params: FetchComposerActionsParams,
) -> Result<models::CastComposerActionsListResponse, Error<FetchComposerActionsError>> {
    let uri_str = format!(
        "{}/farcaster/cast/composer_actions/list",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("list", &params.list.to_string())]);
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CastComposerActionsListResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CastComposerActionsListResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchComposerActionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Crawls the given URL and returns metadata useful when embedding the URL in a cast.
pub async fn fetch_embedded_url_metadata(
    configuration: &configuration::Configuration,
    params: FetchEmbeddedUrlMetadataParams,
) -> Result<models::CastEmbedCrawlResponse, Error<FetchEmbeddedUrlMetadataError>> {
    let uri_str = format!("{}/farcaster/cast/embed/crawl", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("url", &params.url.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CastEmbedCrawlResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CastEmbedCrawlResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<FetchEmbeddedUrlMetadataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Gets information about an individual cast by passing in a Farcaster web URL or cast hash
pub async fn lookup_cast_by_hash_or_warpcast_url(
    configuration: &configuration::Configuration,
    params: LookupCastByHashOrWarpcastUrlParams,
) -> Result<models::CastResponse, Error<LookupCastByHashOrWarpcastUrlError>> {
    let uri_str = format!("{}/farcaster/cast", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("identifier", &params.identifier.to_string())]);
    req_builder = req_builder.query(&[("type", &params.r#type.to_string())]);
    if let Some(ref param_value) = params.viewer_fid {
        req_builder = req_builder.query(&[("viewer_fid", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_neynar_experimental {
        req_builder = req_builder.header("x-neynar-experimental", param_value.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CastResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CastResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LookupCastByHashOrWarpcastUrlError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Gets all casts related to a conversation surrounding a cast by passing in a cast hash or Farcaster URL. Includes all the ancestors of a cast up to the root parent in a chronological order. Includes all direct_replies to the cast up to the reply_depth specified in the query parameter.
pub async fn lookup_cast_conversation(
    configuration: &configuration::Configuration,
    params: LookupCastConversationParams,
) -> Result<models::Conversation, Error<LookupCastConversationError>> {
    let uri_str = format!("{}/farcaster/cast/conversation", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("identifier", &params.identifier.to_string())]);
    req_builder = req_builder.query(&[("type", &params.r#type.to_string())]);
    if let Some(ref param_value) = params.reply_depth {
        req_builder = req_builder.query(&[("reply_depth", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.include_chronological_parent_casts {
        req_builder = req_builder.query(&[(
            "include_chronological_parent_casts",
            &param_value.to_string(),
        )]);
    }
    if let Some(ref param_value) = params.viewer_fid {
        req_builder = req_builder.query(&[("viewer_fid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sort_type {
        req_builder = req_builder.query(&[("sort_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.fold {
        req_builder = req_builder.query(&[("fold", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_neynar_experimental {
        req_builder = req_builder.header("x-neynar-experimental", param_value.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Conversation`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Conversation`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LookupCastConversationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Posts a cast or cast reply. Works with mentions and embeds.   (In order to post a cast `signer_uuid` must be approved)
pub async fn publish_cast(
    configuration: &configuration::Configuration,
    params: PublishCastParams,
) -> Result<models::PostCastResponse, Error<PublishCastError>> {
    let uri_str = format!("{}/farcaster/cast", configuration.base_path);
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
    req_builder = req_builder.json(&params.post_cast_req_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostCastResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostCastResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PublishCastError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Search for casts based on a query string, with optional AND filters
pub async fn search_casts(
    configuration: &configuration::Configuration,
    params: SearchCastsParams,
) -> Result<models::CastsSearchResponse, Error<SearchCastsError>> {
    let uri_str = format!("{}/farcaster/cast/search", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("q", &params.q.to_string())]);
    if let Some(ref param_value) = params.mode {
        req_builder = req_builder.query(&[("mode", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.sort_type {
        req_builder = req_builder.query(&[("sort_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.author_fid {
        req_builder = req_builder.query(&[("author_fid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.viewer_fid {
        req_builder = req_builder.query(&[("viewer_fid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.parent_url {
        req_builder = req_builder.query(&[("parent_url", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.channel_id {
        req_builder = req_builder.query(&[("channel_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.priority_mode {
        req_builder = req_builder.query(&[("priority_mode", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_neynar_experimental {
        req_builder = req_builder.header("x-neynar-experimental", param_value.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CastsSearchResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CastsSearchResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SearchCastsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
