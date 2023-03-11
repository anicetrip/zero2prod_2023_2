use secrecy::ExposeSecret;
use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::setup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("failed to connect to postgres.");
        let address=format!(
            "{}:{}",
            configuration.application.host,configuration.application.port
            );
    println!("{}", address);
    let listerner = TcpListener::bind(address)?;
    run(listerner, connection_pool)?.await
}
