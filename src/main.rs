use std::{io::stdout, net::TcpListener};

use env_logger::Env;
use sqlx::{PgConnection, PgPool};
use zerotoprod::configuration::get_configuration;
use zerotoprod::telemetry::{init_subscriber, get_subscriber};
use zerotoprod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subsriber = get_subscriber("zerotoprod".into(), "info".into());
    init_subscriber(subsriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to run Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port.0);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener, connection_pool)?.await
}
