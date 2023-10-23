use std::fs;
use std::time::SystemTime;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html};
use log::info;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_derive::{Deserialize, Serialize};
use crate::ApplicationState;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Car {
    brand: String,
    color: String,
    fuel: String,
}

pub(crate) async fn is_alive(State(application_state): State<ApplicationState>) -> (StatusCode, &'static str) {
    match application_state.alive {
        true => {
            (StatusCode::OK, "I'm alive! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "I'm dead x_x")
    }
}


pub(crate) async fn is_ready(State(application_state): State<ApplicationState>) -> (StatusCode, &'static str) {
    match application_state.ready {
        true => {
            (StatusCode::OK, "I'm ready! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Please wait! I'm not ready :(")
    }
}

pub(crate) async fn cars() -> Json<Vec<Car>> {
    let sys_time_start = SystemTime::now();

    let mut cars = Vec::new();
    let mut rng = thread_rng();

    for _n in 0..20 {
        cars.push(Car {
            brand: ["Bmw".to_string(), "Volvo".to_string(), "Volkswagen".to_string()].choose(&mut rng).unwrap().to_string(),
            color: ["Blue".to_string(), "Black".to_string(), "White".to_string(), "Red".to_string()].choose(&mut rng).unwrap().to_string(),
            fuel: ["Electric".to_string(), "Disel".to_string(), "Gasoline".to_string()].choose(&mut rng).unwrap().to_string(),
        });
    }
    let sys_time_end = SystemTime::now();
    let difference = sys_time_end.duration_since(sys_time_start);

    info!("cars endpoint call took time: {:?}", difference.unwrap());

    Json(cars)
}

pub(crate) async fn my_car() -> Json<Car> {
    info!("my_car endpoint i called");

    let car = Car {
        brand: "Volkswagen".to_string(),
        color: "Black".to_string(),
        fuel: "Electric".to_string(),
    };
    Json(car)
}

pub(crate) async  fn root() -> Html<&'static str> {
    let html_in_string: String = fs::read_to_string("static/index.html").expect("failed to pare html file to string");
    let html_in_str: &str = string_to_static_str(html_in_string);

    Html(html_in_str)
}

fn string_to_static_str(s: String) -> &'static str {
    s.leak()
}