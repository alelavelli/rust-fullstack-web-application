use backend::{EnvironmentService, EnvironmentServiceTrait};
use tracing::info;

#[tokio::main]
async fn main() {
    let environment_service = EnvironmentService::new();

    // initialize tracing logging with level defined by the environment service
    tracing_subscriber::fmt()
        .with_max_level(environment_service.get_logging_level())
        .with_ansi(true)
        .init();

    info!("Hello!");
}
