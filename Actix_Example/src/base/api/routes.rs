use super::extensions::{endpoint::UserEndpoint, websocket::WebSocketEndpoint};
use actix_web::web;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::get().to(UserEndpoint::list))
            .route("/users", web::post().to(UserEndpoint::create))
            .route("/ws", web::get().to(WebSocketEndpoint::handler)),
    );
}
