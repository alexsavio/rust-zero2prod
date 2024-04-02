use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // setup tracing
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // setup database connection
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_lazy_with(configuration.database.with_db()
    );
    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port,
    );
    let listener = TcpListener::bind(address)?;

    // run the server
    run(listener, connection_pool)?.await?;
    Ok(())
}

