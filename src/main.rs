mod api;
mod router;
mod application_state;
mod log;

use ::log::info;
use crate::application_state::ApplicationState;
use crate::log::init_log4rs;
use crate::router::create_router;


#[tokio::main]
async fn main() {
    init_log4rs();

    let application_state = ApplicationState {
        alive: true,
        ready: true,
    };

    let app = create_router(application_state);

    let listener =
        tokio::net::TcpListener::bind(&"0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    info!("Server has started");
}
