use actix_web::{http::header, App, HttpServer, web};
use actix_cors::Cors;
use sqlx::mysql::MySqlPoolOptions;
use crate::settings::Settings;

mod api;
mod settings;

const CURRENT_DIR: &str = "./config/Settings.toml";

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let s = Settings::new(CURRENT_DIR).unwrap();
    let ip = s.server.get_ip();
    let url = s.database.url;
    let pool_size = s.database.pool_size;

    let pool = MySqlPoolOptions::new()
        .max_connections(pool_size)
        .connect(&url).await?;

    println!("server listen {}", ip);
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .service(
            web::scope("/api")
                .app_data(web::Data::new(pool.clone()))
                .service(api::links::create_link)
                .service(api::links::get_from_link)
                .service(api::links::list_link)
        )
    })
        .bind(&ip)?
        .run()
        .await?;

    Ok(())
}