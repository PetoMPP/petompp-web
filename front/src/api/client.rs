use std::fmt::Display;

use reqwasm::http::*;
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use serde_json::Value;

use crate::models::{credentials::Credentials, user::User};

#[derive(Debug)]
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
        // Response body
        // {
        //   "status": "success" | "error",
        //   "data": T | String,
        // }
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

    async fn post_json<R: DeserializeOwned>(path: &str, body: &impl Serialize) -> Result<R, Error> {
        let request = Request::new(Self::get_url(path).as_str())
            .method(Method::POST)
            .body(serde_json::to_string(body).map_err(|e| Error::Parse(e.to_string()))?);

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
        Self::post_json("api/v1/users/login", &credentials).await
    }

    pub async fn register(credentials: Credentials) -> Result<(), Error> {
        Self::post_json::<User>("api/v1/users", &credentials)
            .await
            .map(|_| ())
    }
}
