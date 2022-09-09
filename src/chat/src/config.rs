use actix_web::web::Data;

#[derive(Debug, Clone)]
pub struct Config {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: String,
    pub auth_endpoint: String,
    pub token_endpoint: String,
}

pub const GRANT_TYPE_KEY: &str = "GrantType";
pub const CLIENT_ID_KEY: &str = "ClientId";
pub const CLIENT_SECRET_KEY: &str = "ClientSecret";
pub const REDIRECT_URI_KEY: &str = "RedirectUri";
pub const SCOPE_KEY: &str = "Scope";
pub const AUTH_ENDPOINT_KEY: &str = "Auth_Endpoint";
pub const TOKEN_ENDPOINT_KEY: &str = "Token_Endpoint";

pub fn load_config() -> Data<Config> {
    dotenv::dotenv().ok();
    Data::new(Config {
        grant_type: dotenv::var(GRANT_TYPE_KEY).unwrap(),
        client_id: dotenv::var(CLIENT_ID_KEY).unwrap(),
        client_secret: dotenv::var(CLIENT_SECRET_KEY).unwrap(),
        redirect_uri: dotenv::var(REDIRECT_URI_KEY).unwrap(),
        scope: dotenv::var(SCOPE_KEY).unwrap(),
        auth_endpoint: dotenv::var(AUTH_ENDPOINT_KEY).unwrap(),
        token_endpoint: dotenv::var(TOKEN_ENDPOINT_KEY).unwrap(),
    })
}
