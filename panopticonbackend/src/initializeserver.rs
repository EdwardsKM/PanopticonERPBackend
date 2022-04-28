use crate::handlers::handlers::{
    absa_bank_handlers::*, bill_details_handlers::*, cfc_handlers::*,
    collection_details_handlers::*, dashboard, health_check, index, lab_visits_handlers::*,
    mpesa_handlers::*, mtiba_handlers::*, pdq_handlers::*, registered_patients_handlers::*,
    sidian_handlers::*,
};

use crate::configs::config::Config;

use rustls::ServerConfig;

use deadpool_postgres::Pool;

use actix_cors::Cors;
use actix_web::middleware::Compress;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub async fn initialize_server(
    pool: Pool,
    rustlsconfig: ServerConfig,
    config: Config,
) -> std::io::Result<()> {
    // Instantiate the Actix-Web Server
    let server = HttpServer::new(move || {
        App::new()
            // Logger must be initialized before the other services to collect all data
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .wrap(Compress::default())
            // Allows us to pass app state to handlers. In this case, the db pool
            .app_data(web::Data::new(pool.clone()))
            // Set the maximum payload size to 32MB
            .app_data(web::PayloadConfig::new(1 << 25))
            .service(health_check)
            .service(get_mpesa_statement)
            .service(get_collection_details)
            .service(get_bill_details)
            .service(get_lab_visits)
            .service(get_registered_patients)
            .service(get_mtiba_statement)
            .service(get_sidian_bank_statement)
            .service(get_cfc_bank_statement)
            .service(get_absa_bank_statement)
            .service(get_pdq_statement)
            .service(update_mpesa_statement)
            .service(update_collection_details)
            .service(update_bill_details)
            .service(update_mtiba_statement)
            .service(update_cfc_statement)
            .service(update_pdq_breakdowns)
            .service(update_absa_statement)
            .service(update_sidian_statement)
            .service(update_lab_visits)
            .service(index)
            .service(dashboard)
    })
    .bind_rustls(config.server_addr.clone(), rustlsconfig)?
    .run();

    println!("Server running at http://{}/", config.server_addr);

    server.await?;

    // Ensure all spans have been shipped to Jaeger.
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
