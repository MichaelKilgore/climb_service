use std::env;
use std::io;

use actix_rt::main;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware;
use crate::activities::create_climb_user::create_climb_user;

use crate::activities::create_climbing_location::create_climbing_location;
use crate::activities::hello_world::hello;

mod activities;
mod model;
mod utils;
mod errors;

#[main]
async fn main() -> io::Result<()>  {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(create_climbing_location)
            .service(create_climb_user)
            .service(hello)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
