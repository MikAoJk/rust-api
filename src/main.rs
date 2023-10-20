mod api;

use axum::{ Router};
use axum::routing::get;
use log::info;
use serde_derive::{Deserialize, Serialize};
use crate::api::{cars, is_alive, is_ready, my_car, root};


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {

    let application_state = ApplicationState {
        alive: true,
        ready: true,
    };

    let router = Router::new()
        .route("/", get(root()))
        .route("/is_alive", get(is_alive(application_state)))
        .route("/is_ready", get(is_ready(application_state)))
        .route("/cars", get(cars()))
        .route("/my_car", get(my_car()));

    let listener =
        tokio::net::TcpListener::bind(&"0.0.0.0:8080").await?;

    info!("listening on {}", listener.local_addr()?);

    axum::Server::from_tcp(listener.into_std()?)?
        .serve(router.into_make_service())
        .await?;

    Ok(())

}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    alive: bool,
    ready: bool,
}