extern crate dotenv;

use std::sync::Mutex;

use actix_files as fs;
use actix_web::middleware::{Compress, Logger};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use settings::Settings;

mod components;
mod routes;
mod settings;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  env_logger::init_from_env(Env::default().default_filter_or("info"));

  let settings = Settings::from_env();
  let data = Data::new(Mutex::new(settings.clone()));

  HttpServer::new(move || {
    App::new()
      .app_data(data.clone())
      .wrap(Logger::default())
      .wrap(Compress::default())
      .service(
        fs::Files::new("/static", "./apps/front/static")
          .show_files_listing()
          .use_last_modified(true),
      )
      .service(web::scope("").configure(routes::configure))
  })
  .bind(format!("0.0.0.0:{}", settings.port))?
  .run()
  .await
}
