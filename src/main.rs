use actix_web::{App, HttpServer, middleware::Logger, web};
use rs_crud_actix::{app_config, metrics};
extern crate env_logger;

mod settings;

#[allow(unused)]
struct AppDataEnv {
    env_settings: settings::Settings
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()>{
    /*
        setup a logger
     */
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config_env = settings::Settings::new().unwrap();


    HttpServer::new(|| {
        let  promethesus_metrics = metrics::create_app_metrics();
        App::new()
            .app_data(web::Data::new(AppDataEnv{
                env_settings: settings::Settings::new().unwrap()
            }))
            .wrap(promethesus_metrics)
            .configure(app_config)
            .wrap(Logger::new("%t: %a %{User-Agent}i"))
    })
        .bind((config_env.server.ip, 8000))?
        .run()
        .await
}