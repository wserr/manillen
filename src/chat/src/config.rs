use actix_web::web::Data;
use log::{error, info};
use std::env;

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
    Data::new(Config {
        grant_type: get_env_var(GRANT_TYPE_KEY, true),
        client_id: get_env_var(CLIENT_ID_KEY, true),
        client_secret: get_env_var(CLIENT_SECRET_KEY, true),
        redirect_uri: get_env_var(REDIRECT_URI_KEY, true),
        scope: get_env_var(SCOPE_KEY, true),
        auth_endpoint: get_env_var(AUTH_ENDPOINT_KEY, true),
        token_endpoint: get_env_var(TOKEN_ENDPOINT_KEY, true),
    })
}

fn get_env_var(key: &str, required: bool) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            if required {
                error!("Could not read Key {}. Reason: {}", key, e);
            } else {
                info!("Could not read Key {}. Reason: {}", key, e);
            }
            String::new()
        }
    }
}
