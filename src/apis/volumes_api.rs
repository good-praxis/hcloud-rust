/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `attach_volume_to_server`
#[derive(Clone, Debug, Default)]
pub struct AttachVolumeToServerParams {
    /// ID of the Volume
    pub id: i32,
    pub attach_volume_to_server_request: Option<crate::models::AttachVolumeToServerRequest>
}

/// struct for passing parameters to the method `change_volume_protection`
#[derive(Clone, Debug, Default)]
pub struct ChangeVolumeProtectionParams {
    /// ID of the Volume
    pub id: i32,
    pub change_volume_protection_request: Option<crate::models::ChangeVolumeProtectionRequest>
}

/// struct for passing parameters to the method `create_volume`
#[derive(Clone, Debug, Default)]
pub struct CreateVolumeParams {
    pub create_volume_request: Option<crate::models::CreateVolumeRequest>
}

/// struct for passing parameters to the method `delete_volume`
#[derive(Clone, Debug, Default)]
pub struct DeleteVolumeParams {
    /// ID of the Volume
    pub id: String
}

/// struct for passing parameters to the method `detach_volume`
#[derive(Clone, Debug, Default)]
pub struct DetachVolumeParams {
    /// ID of the Volume
    pub id: i32
}

/// struct for passing parameters to the method `get_action_for_volume`
#[derive(Clone, Debug, Default)]
pub struct GetActionForVolumeParams {
    /// ID of the Volume
    pub id: i32,
    /// ID of the Action
    pub action_id: i32
}

/// struct for passing parameters to the method `get_volume`
#[derive(Clone, Debug, Default)]
pub struct GetVolumeParams {
    /// ID of the Volume
    pub id: i32
}

/// struct for passing parameters to the method `list_actions_for_volume`
#[derive(Clone, Debug, Default)]
pub struct ListActionsForVolumeParams {
    /// ID of the Volume
    pub id: i32,
    /// Can be used multiple times.
    pub sort: Option<String>,
    /// Can be used multiple times, the response will contain only Actions with specified statuses
    pub status: Option<String>,
    /// Specifies the page to fetch. The number of the first page is 1
    pub page: Option<i32>,
    /// Specifies the number of items returned per page. The default value is 25, the maximum value is 50 except otherwise specified in the documentation.
    pub per_page: Option<i32>
}

/// struct for passing parameters to the method `list_volumes`
#[derive(Clone, Debug, Default)]
pub struct ListVolumesParams {
    /// Can be used multiple times. The response will only contain Volumes matching the status.
    pub status: Option<String>,
    /// Can be used multiple times.
    pub sort: Option<String>,
    /// Can be used to filter resources by their name. The response will only contain the resources matching the specified name
    pub name: Option<String>,
    /// Can be used to filter resources by labels. The response will only contain resources matching the label selector.
    pub label_selector: Option<String>,
    /// Specifies the page to fetch. The number of the first page is 1
    pub page: Option<i32>,
    /// Specifies the number of items returned per page. The default value is 25, the maximum value is 50 except otherwise specified in the documentation.
    pub per_page: Option<i32>
}

/// struct for passing parameters to the method `replace_volume`
#[derive(Clone, Debug, Default)]
pub struct ReplaceVolumeParams {
    /// ID of the Volume to update
    pub id: String,
    pub replace_volume_request: Option<crate::models::ReplaceVolumeRequest>
}

/// struct for passing parameters to the method `resize_volume`
#[derive(Clone, Debug, Default)]
pub struct ResizeVolumeParams {
    /// ID of the Volume
    pub id: i32,
    pub resize_volume_request: Option<crate::models::ResizeVolumeRequest>
}


/// struct for typed errors of method `attach_volume_to_server`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachVolumeToServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `change_volume_protection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeVolumeProtectionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateVolumeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteVolumeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `detach_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetachVolumeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_action_for_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForVolumeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVolumeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_actions_for_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForVolumeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_volumes`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVolumesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceVolumeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `resize_volume`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResizeVolumeError {
    UnknownValue(serde_json::Value),
}


/// Attaches a Volume to a Server. Works only if the Server is in the same Location as the Volume.
pub async fn attach_volume_to_server(configuration: &configuration::Configuration, params: AttachVolumeToServerParams) -> Result<crate::models::AttachVolumeToServerResponse, Error<AttachVolumeToServerError>> {
    // unbox the parameters
    let id = params.id;
    let attach_volume_to_server_request = params.attach_volume_to_server_request;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}/actions/attach", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&attach_volume_to_server_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AttachVolumeToServerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Changes the protection configuration of a Volume.
pub async fn change_volume_protection(configuration: &configuration::Configuration, params: ChangeVolumeProtectionParams) -> Result<crate::models::ChangeVolumeProtectionResponse, Error<ChangeVolumeProtectionError>> {
    // unbox the parameters
    let id = params.id;
    let change_volume_protection_request = params.change_volume_protection_request;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}/actions/change_protection", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&change_volume_protection_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeVolumeProtectionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new Volume attached to a Server. If you want to create a Volume that is not attached to a Server, you need to provide the `location` key instead of `server`. This can be either the ID or the name of the Location this Volume will be created in. Note that a Volume can be attached to a Server only in the same Location as the Volume itself.  Specifying the Server during Volume creation will automatically attach the Volume to that Server after it has been initialized. In that case, the `next_actions` key in the response is an array which contains a single `attach_volume` action.  The minimum Volume size is 10GB and the maximum size is 10TB (10240GB).  A volume’s name can consist of alphanumeric characters, dashes, underscores, and dots, but has to start and end with an alphanumeric character. The total length is limited to 64 characters. Volume names must be unique per Project.  #### Call specific error codes  | Code                                | Description                                                          | |-------------------------------------|----------------------------------------------------------------------| | `no_space_left_in_location`         | There is no volume space left in the given location                    | 
pub async fn create_volume(configuration: &configuration::Configuration, params: CreateVolumeParams) -> Result<crate::models::CreateVolumeResponse, Error<CreateVolumeError>> {
    // unbox the parameters
    let create_volume_request = params.create_volume_request;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_volume_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a volume. All Volume data is irreversibly destroyed. The Volume must not be attached to a Server and it must not have delete protection enabled.
pub async fn delete_volume(configuration: &configuration::Configuration, params: DeleteVolumeParams) -> Result<(), Error<DeleteVolumeError>> {
    // unbox the parameters
    let id = params.id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Detaches a Volume from the Server it’s attached to. You may attach it to a Server again at a later time.
pub async fn detach_volume(configuration: &configuration::Configuration, params: DetachVolumeParams) -> Result<crate::models::DetachVolumeResponse, Error<DetachVolumeError>> {
    // unbox the parameters
    let id = params.id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}/actions/detach", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DetachVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific Action for a Volume.
pub async fn get_action_for_volume(configuration: &configuration::Configuration, params: GetActionForVolumeParams) -> Result<crate::models::GetActionForVolumeResponse, Error<GetActionForVolumeError>> {
    // unbox the parameters
    let id = params.id;
    let action_id = params.action_id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}/actions/{action_id}", configuration.base_path, id=id, action_id=action_id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetActionForVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a specific Volume object.
pub async fn get_volume(configuration: &configuration::Configuration, params: GetVolumeParams) -> Result<crate::models::GetVolumeResponse, Error<GetVolumeError>> {
    // unbox the parameters
    let id = params.id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Action objects for a Volume. You can `sort` the results by using the sort URI parameter, and filter them with the `status` parameter.
pub async fn list_actions_for_volume(configuration: &configuration::Configuration, params: ListActionsForVolumeParams) -> Result<crate::models::ListActionsForVolumeResponse, Error<ListActionsForVolumeError>> {
    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}/actions", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListActionsForVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets all existing Volumes that you have available.
pub async fn list_volumes(configuration: &configuration::Configuration, params: ListVolumesParams) -> Result<crate::models::ListVolumesResponse, Error<ListVolumesError>> {
    // unbox the parameters
    let status = params.status;
    let sort = params.sort;
    let name = params.name;
    let label_selector = params.label_selector;
    let page = params.page;
    let per_page = params.per_page;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("label_selector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListVolumesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the Volume properties.  Note that when updating labels, the volume’s current set of labels will be replaced with the labels provided in the request body. So, for example, if you want to add a new label, you have to provide all existing labels plus the new label in the request body. 
pub async fn replace_volume(configuration: &configuration::Configuration, params: ReplaceVolumeParams) -> Result<crate::models::ReplaceVolumeResponse, Error<ReplaceVolumeError>> {
    // unbox the parameters
    let id = params.id;
    let replace_volume_request = params.replace_volume_request;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_volume_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Changes the size of a Volume. Note that downsizing a Volume is not possible.
pub async fn resize_volume(configuration: &configuration::Configuration, params: ResizeVolumeParams) -> Result<crate::models::ResizeVolumeResponse, Error<ResizeVolumeError>> {
    // unbox the parameters
    let id = params.id;
    let resize_volume_request = params.resize_volume_request;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/volumes/{id}/actions/resize", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&resize_volume_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ResizeVolumeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

