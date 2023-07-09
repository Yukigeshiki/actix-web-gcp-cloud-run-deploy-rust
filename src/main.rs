use std::net::TcpListener;

use actix_web_cloud_run::config::get_config;
use actix_web_cloud_run::run;
use actix_web_cloud_run::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("actix-web-sqlx-test".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // stop application if get config or db connect fail
    let config = get_config().expect("Failed to read configuration.");
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(addr)?;
    run(listener)?.await?;
    Ok(())
}
