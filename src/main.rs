use std::fs;
use axum::http::StatusCode;
use axum::response::Html;
use axum::{Json, Router};
use axum::routing::get;
use log::info;
use serde_derive::{Deserialize, Serialize};
use rand::{thread_rng};
use rand::prelude::SliceRandom;


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {

    let application_state = ApplicationState {
        alive: true,
        ready: true,
    };

    let app = Router::new()
        .route("/", get(root()))
        .route("/is_alive", get(is_alive(application_state)))
        .route("/is_ready", get(is_ready(application_state)))
        .route("/cars", get(cars()));
    let listener =
        tokio::net::TcpListener::bind(&"0.0.0.0:8080").await?;

    info!("listening on {}", listener.local_addr()?);

    axum::Server::from_tcp(listener.into_std()?)?
        .serve(app.into_make_service())
        .await?;

    Ok(())

}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    alive: bool,
    ready: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Car {
    color: String,
    fuel: String,
}

fn is_alive(application_state: ApplicationState) -> (StatusCode, &'static str) {
    match application_state.alive {
        true => {
            (StatusCode::OK, "I'm alive! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "I'm dead x_x")
    }
}


fn is_ready(application_state: ApplicationState) -> (StatusCode, &'static str) {
    match application_state.ready {
        true => {
            (StatusCode::OK, "I'm ready! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Please wait! I'm not ready :(")
    }
}

fn cars() -> Json<Vec<Car>> {

    let mut cars = Vec::new();
    let mut rng = thread_rng();

    for _n in 0..20 {
        cars.push(Car {
            color: ["Blue".to_string(),"Black".to_string(),"White".to_string(),"Red".to_string()].choose(&mut rng).unwrap().to_string(),
            fuel: ["Electric".to_string(),"Disel".to_string(),"Gasoline".to_string()].choose(&mut rng).unwrap().to_string(),
        });
    }

    Json(cars)
}


fn root() -> Html<&'static str> {
    let html_in_string: String = fs::read_to_string("static/index.html").expect("failed to pare html file to string");
    let html_in_str: &str = string_to_static_str(html_in_string);

    Html(html_in_str)
}

fn string_to_static_str(s: String) -> &'static str {
    s.leak()
}
