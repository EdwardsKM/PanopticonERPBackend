mod configs;
mod errors;
mod handlers;
mod https_config;
mod initializeserver;
mod models;
mod telemetry;

use crate::https_config::rustls_config::load_rustls_config;
use crate::initializeserver::initialize_server;
use crate::telemetry::telemetry::init_telemetry;
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    init_telemetry();

    // Include environment variables
    dotenv().ok();

    //Create Tls config
    let rustlsconfig = load_rustls_config();

    // Use the config module to set up the database pool with environment variables
    let config = crate::configs::config::Config::from_env().unwrap();

    // Create the pool using Deadpool Postgres
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    initialize_server(pool, rustlsconfig, config).await
}
