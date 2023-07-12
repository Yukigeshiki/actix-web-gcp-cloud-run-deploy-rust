use std::net::TcpListener;

use actix_web_cloud_run::run;
use actix_web_cloud_run::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("actix-web-cloud-run".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let listener = TcpListener::bind("0.0.0.0:8080".to_string())?;
    run(listener)?.await?;
    Ok(())
}
