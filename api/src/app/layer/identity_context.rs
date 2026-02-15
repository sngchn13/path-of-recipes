use axum::{debug_middleware, extract::{FromRequestParts, Request, State}, http::{StatusCode, request::Parts}, middleware::Next, response::{IntoResponse, Response}};
use axum_extra::extract::{CookieJar, cookie::{Cookie, SameSite}};
use uuid::Uuid;

use crate::app::state::AppConfig;


const COOKIE_KEY: &str = "anon_id";

#[derive(Clone, Debug)]
pub struct AnonId(pub String);

#[derive(Debug)]
pub struct MissingAnonId;

impl IntoResponse for MissingAnonId {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "anon_id not initialized").into_response()
    }
}

impl<S> FromRequestParts<S> for AnonId 
   where S: Send + Sync, 
{
    type Rejection = MissingAnonId;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<AnonId>().cloned().ok_or(MissingAnonId)
    }
}

#[debug_middleware]
pub async fn anon_id_middleware(State(config): State<AppConfig>, jar: CookieJar, mut request: Request, next: Next) -> Response{

    let (anon_id, need_set_cookie) = match jar.get(COOKIE_KEY) {
        Some(c) => (c.value().to_string(), false),
        None => (Uuid::new_v4().to_string(), true),
    };

    request.extensions_mut().insert(AnonId(anon_id.clone()));

    let response = next.run(request).await;

    if !need_set_cookie {
        return response;
    }

    let cookie = Cookie::build((COOKIE_KEY, anon_id))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(365))
        .secure(config.cookie_secure)
        .build();
    let jar = jar.add(cookie);

    (jar, response).into_response()
}