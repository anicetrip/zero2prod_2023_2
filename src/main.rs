use std::{net::TcpListener, fmt::format};

use sqlx::{PgConnection, Connection, PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::setup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("failed to connect to postgres.");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    println!("{}",address);
    let listerner = TcpListener::bind(address)?;
    run(listerner,connection_pool)?.await
}

