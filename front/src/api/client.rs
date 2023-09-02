use std::fmt::Display;

use reqwasm::http::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::models::{credentials::Credentials, user::User, resource_data::ResourceData};

#[derive(Debug, PartialEq)]
pub enum Error {
    Endpoint(u16, String),
    Parse(String),
    Network(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for Error {}

pub enum Response<T> {
    Success(T),
    Error(u16, String),
}

impl<T: DeserializeOwned> Response<T> {
    async fn from_response(value: reqwasm::http::Response) -> Result<Self, Error> {
        let status_code = value.status();
        let body = value
            .json::<Value>()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
        let status = body
            .get("status")
            .ok_or(Error::Parse("Missing status".to_string()))?
            .as_str()
            .ok_or(Error::Parse("Invalid status format".to_string()))?;
        let data = body
            .get("data")
            .ok_or(Error::Parse("Missing data".to_string()))?;
        match status {
            "success" => Ok(Response::Success(
                serde_json::from_value(data.clone())
                    .map_err(|e| Error::Parse(format!("Invalid data format, {}", e)))?,
            )),
            "error" => Ok(Response::Error(
                status_code,
                data.as_str()
                    .ok_or(Error::Parse("Invalid error data format".to_string()))?
                    .to_string(),
            )),
            _ => Err(Error::Parse("Invalid status".to_string())),
        }
    }
}

pub struct Client;

lazy_static::lazy_static! {
    static ref BASE_URL: &'static str = std::option_env!("API_URL").unwrap_or("http://localhost:16969");
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

impl Client {
    fn get_url(path: &str) -> String {
        let separator = if path.starts_with("/") { "" } else { "/" };
        format!("{}{}{}", *BASE_URL, separator, path)
    }

    async fn send_json<R: DeserializeOwned>(
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<&impl Serialize>,
    ) -> Result<R, Error> {
        let mut request = Request::new(Self::get_url(path).as_str()).method(method);
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token).as_str());
        }
        if let Some(body) = body {
            request =
                request.body(serde_json::to_string(body).map_err(|e| Error::Parse(e.to_string()))?);
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        match Response::from_response(response).await? {
            Response::Success(data) => Ok(data),
            Response::Error(s, e) => Err(Error::Endpoint(s, e)),
        }
    }

    pub async fn login(credentials: Credentials) -> Result<LoginResponse, Error> {
        Self::send_json(Method::POST, "api/v1/users/login", None, Some(&credentials)).await
    }

    pub async fn register(credentials: Credentials) -> Result<(), Error> {
        Self::send_json::<User>(Method::POST, "api/v1/users", None, Some(&credentials))
            .await
            .map(|_| ())
    }

    pub async fn get_users(token: &str) -> Result<Vec<User>, Error> {
        Self::send_json(
            Method::GET,
            "api/v1/users/all?range=all",
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|u: Vec<Vec<User>>| u[0].clone())
    }

    pub async fn activate_user(token: &str, id: i32) -> Result<(), Error> {
        Self::send_json::<User>(
            Method::POST,
            format!("api/v1/users/{}/activate", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }

    pub async fn delete_user(token: &str, id: i32) -> Result<(), Error> {
        Self::send_json::<User>(
            Method::DELETE,
            format!("api/v1/users/{}", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }

    pub async fn get_resource(key: &str, lang: &str) -> Result<String, Error> {
        Self::send_json(
            Method::GET,
            format!("api/v1/res/{}?lang={}", key, lang).as_str(),
            None,
            Option::<&String>::None,
        )
        .await
    }

    pub async fn update_resource(
        token: &str,
        key: &str,
        lang: &str,
        value: &str,
    ) -> Result<(), Error> {
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
}
