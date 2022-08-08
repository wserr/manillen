use actix_session::Session;
use actix_web::body::BoxBody;
use actix_web::error::PathError::Deserialize;
use actix_web::get;
use actix_web::http::header::ContentType;
use actix_web::*;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[get("/session-test")]
pub async fn session_test(session: Session) -> Result<impl Responder> {
    println!("{:?}", session.entries());
    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    let count = session.get::<i32>("counter")?.unwrap();
    Ok(format!("Counter: {}", count))
}

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
pub async fn login() -> Result<impl Responder> {
    Ok(HttpResponse::PermanentRedirect()
        .append_header((
            "location",
            "http://manillen.identity.local/auth/realms/manillen/protocol/openid-connect/auth?response_type=code&client_id=manillen&redirect_uri=http://localhost:8080/token_callback&scope=openid&state=123",
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
pub async fn token(info: web::Query<TokenCallbackInfo>) -> Result<impl Responder> {
    // Perform request to fetch token from keycloak

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", "manillen"),
        ("client_secret", "vzUTjJMiBXEhUTsn724m4zd3U9cBgRqV"),
        ("code", info.code.as_str()),
        ("redirect_uri", "http://localhost:8080/token_callback"),
        ("state", info.state.as_str()),
        ("session_state", info.session_state.as_str()),
        ("scope", "openid"),
    ];
    let client = reqwest::Client::new();
    let res = client
        .post("http://manillen.identity.local/auth/realms/manillen/protocol/openid-connect/token")
        .form(&params)
        .send()
        .await;
    let result = res.unwrap().text().await.unwrap();
    Ok(info.state.to_string())
}
