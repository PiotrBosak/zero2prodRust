use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use zerotoprod::configuration::get_configuration;
use zerotoprod::run;
use zerotoprod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subsriber = get_subscriber("zerotoprod".into(), "info".into());
    init_subscriber(subsriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to run Postgres");
    let address = format!(
        "{}:{}",
        configuration.application.host.0, configuration.application.port.0
    );
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener, connection_pool)?.await
}
