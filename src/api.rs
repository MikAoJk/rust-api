use std::fs;
use axum::http::StatusCode;
use axum::Json;
use axum::response::Html;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_derive::{Deserialize, Serialize};
use crate::ApplicationState;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Car {
    color: String,
    fuel: String,
}

pub(crate) fn is_alive(application_state: ApplicationState) -> (StatusCode, &'static str) {
    match application_state.alive {
        true => {
            (StatusCode::OK, "I'm alive! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "I'm dead x_x")
    }
}


pub(crate) fn is_ready(application_state: ApplicationState) -> (StatusCode, &'static str) {
    match application_state.ready {
        true => {
            (StatusCode::OK, "I'm ready! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Please wait! I'm not ready :(")
    }
}

pub(crate) fn cars() -> Json<Vec<Car>> {

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


pub(crate) fn root() -> Html<&'static str> {
    let html_in_string: String = fs::read_to_string("static/index.html").expect("failed to pare html file to string");
    let html_in_str: &str = string_to_static_str(html_in_string);

    Html(html_in_str)
}

fn string_to_static_str(s: String) -> &'static str {
    s.leak()
}