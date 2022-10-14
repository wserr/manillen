use crate::Config;
use actix_web::body::BoxBody;
use actix_web::get;
use actix_web::http::header::ContentType;
use actix_web::*;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserInfo {
    user_name: String,
}

impl Responder for UserInfo {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        // Create response and set content type

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)

        //HttpResponse::PermanentRedirect()
        //    .append_header(("location", "/ws/"))
        //    .body(body)
        //
    }
}

// Add endpoint: userInfo
// The endpoint should fetch the userInfo from the keycloak server
// If there is no id token found in the session cookie, redirect to login page
pub async fn user_info(_req: HttpRequest) -> impl Responder {
    UserInfo {
        user_name: "Willem".to_string(),
    }
}

// Add endpoint: redirect to login screen
#[get("/login")]
pub async fn login(data: web::Data<Config>) -> Result<impl Responder> {
    Ok(HttpResponse::PermanentRedirect()
        .append_header((
            "location",
            format!(
                "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state=123",
                data.auth_endpoint, data.client_id, data.redirect_uri, data.scope
            ),
        ))
        .finish())
}

#[derive(Deserialize)]
pub struct TokenCallbackInfo {
    state: String,
    session_state: String,
    code: String,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    access_token: String,
    id_token: String,
    session_state: String,
    scope: String,
}

// Add endpoint: redirect
#[get("/token_callback")]
pub async fn token(
    data: web::Data<Config>,
    info: web::Query<TokenCallbackInfo>,
) -> Result<impl Responder> {
    // Perform request to fetch token from keycloak

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", data.client_id.as_str()),
        ("client_secret", data.client_secret.as_str()),
        ("code", info.code.as_str()),
        ("redirect_uri", data.redirect_uri.as_str()),
        ("state", info.state.as_str()),
        ("session_state", info.session_state.as_str()),
        ("scope", data.scope.as_str()),
    ];
    let client = reqwest::Client::new();
    let res = client
        .post(data.token_endpoint.as_str())
        .form(&params)
        .send()
        .await;
    Ok(info.state.to_string())
}
