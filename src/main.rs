mod hello_world;

use actix_web::{App, HttpServer, middleware};
use actix_rt::{main};
use std::io;
use hello_world::{hello};
use std::env;

#[main]
async fn main() -> io::Result<()>  {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(hello)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
