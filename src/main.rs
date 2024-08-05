use std::env;
use std::io;

use actix_rt::main;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware;
use crate::activities::create_climb_user::create_climb_user;

use crate::activities::create_climb_location::create_climb_location;
use crate::activities::create_climb_route::create_climb_route;
use crate::activities::hello_world::hello;
use crate::activities::update_climb_user_user_name::update_climb_user_user_name;
use crate::activities::send_verification_code::send_verification_code;
use crate::activities::verify_phone_number::verify_phone_number;

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
            .service(create_climb_location)
            .service(create_climb_user)
            .service(update_climb_user_user_name)
            .service(hello)
            .service(create_climb_route)
            .service(send_verification_code)
            .service(verify_phone_number)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
