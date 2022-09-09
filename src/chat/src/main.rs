use crate::config::{load_config, Config};
use crate::endpoints::{login, token, user_info};
use crate::web_socket::index;
use crate::session::session_test;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{cookie, web, App, HttpServer};

mod config;
mod endpoints;
mod session;
mod web_socket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .build(),
            )
            .app_data(load_config())
            .service(session_test)
            .service(login)
            .service(token)
            .route("/ws/", web::get().to(index))
            .route("/userInfo", web::get().to(user_info))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
