use actix_web::{App, HttpServer, middleware::Logger};
use rs_crud_actix::app_config;
extern crate env_logger;


#[actix_web::main]
pub async fn main() -> std::io::Result<()>{
    /*
        setup a logger
     */
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(app_config)
            .wrap(Logger::new("%t: %a %{User-Agent}i"))
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}