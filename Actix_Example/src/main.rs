mod base;

use actix_web::{middleware::Logger, web, App, HttpServer};
use base::api::routes::register;
use base::database::connection::Db;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let db = Db::connect().await;
    let data = web::Data::new(db);

    let host = std::env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or("8080".into());
    let addr = format!("{host}:{port}");

    log::info!("Running at http://{addr}");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(register)
    })
    .bind(&addr)?
    .run()
    .await
}
