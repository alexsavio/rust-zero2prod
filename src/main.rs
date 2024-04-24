use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // setup tracing
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // setup database connection
    let configuration = get_configuration().expect("Failed to read configuration.");
    let server = Application::build(configuration).await?;
    server.run_until_stopped().await?;
    Ok(())
}
