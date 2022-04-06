use std::net::TcpListener;

use zerotoprod::configuration::get_configuration;
use zerotoprod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port.0);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind port");
    run(listener)?.await
}
