use axum::{Router};
use axum::routing::get;
use crate::api::{cars, is_alive, is_ready, my_car, root};
use crate::ApplicationState;

pub fn create_router(application_state: ApplicationState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/is_alive", get(is_alive))
        .route("/is_ready", get(is_ready))
        .route("/cars", get(cars))
        .route("/my_car", get(my_car))
        .with_state(application_state)
}