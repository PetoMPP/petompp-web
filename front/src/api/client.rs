use super::error::ApiError;
use crate::models::{credentials::Credentials, resource_data::ResourceData, user::User};
use reqwasm::http::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum RequestError {
    Endpoint(u16, ApiError),
    Parse(String),
    Network(String),
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for RequestError {}

pub enum Response<T> {
    Success(T),
    Error(u16, ApiError),
}

impl<T: DeserializeOwned> Response<T> {
    async fn from_response(value: reqwasm::http::Response) -> Result<Self, RequestError> {
        let status_code = value.status();
        let body = value
            .json::<Value>()
            .await
            .map_err(|e| RequestError::Parse(e.to_string()))?;
        let status = body
            .get("status")
            .ok_or(RequestError::Parse("Missing status".to_string()))?
            .as_str()
            .ok_or(RequestError::Parse("Invalid status format".to_string()))?;
        let data = body
            .get("data")
            .ok_or(RequestError::Parse("Missing data".to_string()))?;
        match status {
            "success" => Ok(Response::Success(
                serde_json::from_value(data.clone())
                    .map_err(|e| RequestError::Parse(format!("Invalid data format, {}", e)))?,
            )),
            "error" => Ok(Response::Error(
                status_code,
                serde_json::from_value(data.clone())
                    .map_err(|e| RequestError::Parse(format!("Invalid data format, {}", e)))?,
            )),
            _ => Err(RequestError::Parse("Invalid status".to_string())),
        }
    }
}

pub struct Client;
lazy_static::lazy_static! {
    static ref API_URL: String = match std::option_env!("API_URL").unwrap_or_default() {
        url if url.ends_with('/') => url.to_string(),
        url => format!("{}/", url)
    };
    static ref AZURE_STORAGE_URL: String = match std::option_env!("AZURE_STORAGE_URL").unwrap_or_default() {
        url if url.ends_with('/') => url.to_string(),
        url => format!("{}/", url)
    };
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

impl Client {
    fn get_api_url(path: &str) -> String {
        format!("{}{}", *API_URL, path)
    }

    async fn send_json<R: DeserializeOwned>(
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<&impl Serialize>,
    ) -> Result<R, RequestError> {
        let mut request = Request::new(Self::get_api_url(path).as_str()).method(method);
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token).as_str());
        }
        if let Some(body) = body {
            request = request
                .body(serde_json::to_string(body).map_err(|e| RequestError::Parse(e.to_string()))?);
        }

        let response = request
            .send()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;

        match Response::from_response(response).await? {
            Response::Success(data) => Ok(data),
            Response::Error(s, e) => Err(RequestError::Endpoint(s, e)),
        }
    }

    pub async fn login(credentials: Credentials) -> Result<LoginResponse, RequestError> {
        Self::send_json(Method::POST, "api/v1/users/login", None, Some(&credentials)).await
    }

    pub async fn register(credentials: Credentials) -> Result<(), RequestError> {
        Self::send_json::<User>(Method::POST, "api/v1/users", None, Some(&credentials))
            .await
            .map(|_| ())
    }

    pub async fn get_users(token: &str) -> Result<Vec<User>, RequestError> {
        Self::send_json(
            Method::GET,
            "api/v1/users/all?range=all",
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|u: Vec<Vec<User>>| u[0].clone())
    }

    pub async fn activate_user(token: &str, id: i32) -> Result<(), RequestError> {
        Self::send_json::<User>(
            Method::POST,
            format!("api/v1/users/{}/activate", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }

    pub async fn delete_user(token: &str, id: i32) -> Result<(), RequestError> {
        Self::send_json::<User>(
            Method::DELETE,
            format!("api/v1/users/{}", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }

    pub async fn get_resource(key: &str, lang: &str) -> Result<String, RequestError> {
        Self::send_json(
            Method::GET,
            format!("api/v1/res/{}?lang={}", key, lang).as_str(),
            None,
            Option::<&String>::None,
        )
        .await
    }

    pub async fn get_resource_keys(token: &str) -> Result<Vec<String>, RequestError> {
        Self::send_json(
            Method::GET,
            "api/v1/res/keys",
            Some(token),
            Option::<&String>::None,
        )
        .await
    }

    pub async fn update_resource(
        token: &str,
        key: &str,
        lang: &str,
        value: &str,
    ) -> Result<(), RequestError> {
        let resource = ResourceData::new_from_lang(key, lang, value)?;
        Self::send_json(
            Method::POST,
            format!("api/v1/res/{}", key).as_str(),
            Some(token),
            Some(&resource),
        )
        .await
        .map(|_: ResourceData| ())
    }

    pub async fn get_locale(lang: &str) -> Result<HashMap<String, String>, RequestError> {
        let resp = Request::new(format!("/locales/{}.yml", lang).as_str())
            .method(Method::GET)
            .send()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;
        let body = resp
            .binary()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;

        serde_yaml::from_slice::<HashMap<String, String>>(&body)
            .map_err(|e| RequestError::Parse(e.to_string()))
    }

    pub async fn upload_img(
        token: &str,
        img: web_sys::File,
        folder: &str,
    ) -> Result<String, RequestError> {
        let resp = Request::new(
            Self::get_api_url(format!("api/v1/img/?folder={}", folder).as_str()).as_str(),
        )
        .method(Method::PUT)
        .header("Authorization", format!("Bearer {}", token).as_str())
        .body(img)
        .send()
        .await
        .map_err(|e| RequestError::Network(e.to_string()))?;
        match Response::<String>::from_response(resp).await? {
            Response::Success(filename) => {
                Ok(format!("{}{}/{}", *AZURE_STORAGE_URL, folder, filename))
            }
            Response::Error(s, e) => Err(RequestError::Endpoint(s, e)),
        }
    }
}
