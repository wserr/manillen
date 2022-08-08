use crate::endpoints::{login, session_test, token, user_info};
use crate::web_socket::index;
use actix_session::storage::CookieSessionStore;
use actix_session::{config, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{cookie, web, App, HttpServer};

mod endpoints;
mod web_socket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .build(),
            )
            .service(session_test)
            .service(login)
            .service(token)
            .route("/ws/", web::get().to(index))
            .route("/userInfo", web::get().to(user_info))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
