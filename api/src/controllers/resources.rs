use crate::{
    auth::claims::AdminClaims,
    controllers::response::ApiResponse,
    models::resource_data::ResourceData,
    repositories::{
        repo::{ApiError, RepoError},
        resources::repo::ResourcesRepo,
    },
};

use super::controller::Controller;
use rocket::{delete, get, post, routes, serde::json::Json, Build};

pub struct ResourcesController;

impl Controller for ResourcesController {
    fn path(&self) -> &'static str {
        "/res"
    }

    fn routes(&self) -> Vec<rocket::Route> {
        routes![get, create, update, delete]
    }

    fn add_managed(&self, rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
        rocket
    }
}

#[get("/<key>?<lang>")]
async fn get<'a>(
    key: &'a str,
    lang: &'a str,
    pool: &dyn ResourcesRepo,
) -> Result<Json<ApiResponse<'a, String>>, ApiError<'a>> {
    Ok(Json(ApiResponse::ok(pool.get(key, lang)?)))
}

#[post("/<key>", data = "<value>")]
async fn create<'a>(
    _admin_claims: AdminClaims,
    key: &'a str,
    value: Json<ResourceData>,
    pool: &dyn ResourcesRepo,
) -> Result<Json<ApiResponse<'a, ResourceData>>, ApiError<'a>> {
    let value = ResourceData {
        key: Some(key.to_string()),
        ..value.into_inner()
    };
    Ok(Json(ApiResponse::ok(pool.create(&value)?)))
}

#[post("/<key>", data = "<value>")]
async fn update<'a>(
    _admin_claims: AdminClaims,
    key: &'a str,
    value: Json<ResourceData>,
    pool: &dyn ResourcesRepo,
) -> Result<Json<ApiResponse<'a, ResourceData>>, ApiError<'a>> {
    if key != value.key.as_ref().unwrap().as_str() {
        return Err(RepoError::ValidationError("Key mismatch.".to_string()).into());
    }
    let value = ResourceData {
        key: Some(key.to_string()),
        ..value.into_inner()
    };
    Ok(Json(ApiResponse::ok(pool.update(&value)?)))
}

#[delete("/<key>")]
async fn delete(
    _admin_claims: AdminClaims,
    key: &str,
    pool: &dyn ResourcesRepo,
) -> Result<&'static str, ApiError<'static>> {
    pool.delete(key)?;
    Ok("OK")
}