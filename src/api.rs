use std::fs;
use std::time::SystemTime;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html};
use log::info;
use rand::prelude::IndexedRandom;
use rand::{rng, Rng};
use serde_derive::{Deserialize, Serialize};
use crate::ApplicationState;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Car {
    brand: String,
    color: String,
    fuel: String,
    hp: u32,
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
    let mut rng = rng();

    for _n in 0..20 {
        cars.push(Car {
            brand: ["Bmw".to_string(), "Volvo".to_string(), "Volkswagen".to_string()].choose(&mut rng).unwrap().to_string(),
            color: ["Blue".to_string(), "Black".to_string(), "White".to_string(), "Red".to_string()].choose(&mut rng).unwrap().to_string(),
            fuel: ["Electric".to_string(), "Disel".to_string(), "Gasoline".to_string()].choose(&mut rng).unwrap().to_string(),
            hp: rng.random_range(80..400)
        });
    }
    let sys_time_end = SystemTime::now();
    let difference = sys_time_end.duration_since(sys_time_start);

    info!("cars endpoint call took time: {:?}", difference.unwrap());

    Json(cars)
}

pub(crate) async fn my_car() -> Json<Car> {
    info!("my_car endpoint i called");
    let sys_time_start = SystemTime::now();


    let car = Car {
        brand: "Volkswagen".to_string(),
        color: "Black".to_string(),
        fuel: "Electric".to_string(),
        hp: 174
    };

    let sys_time_end = SystemTime::now();
    let difference = sys_time_end.duration_since(sys_time_start);

    info!("cars endpoint call took time: {:?}", difference.unwrap());

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

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::{TestResponse, TestServer};
    use crate::api::Car;

    use crate::ApplicationState;
    use crate::router::create_router;


    #[tokio::test]
    async fn configured_liveness_returns_200_when_state_alive() {
        let app = create_router(
            ApplicationState {
                alive: true,
                ready: false,
            }
        );
        let server = TestServer::new(app).unwrap();

        let res = server.get(&"/is_alive".to_string()).await;
        assert_eq!(res.status_code(), StatusCode::OK);

    }

    #[tokio::test]
    async fn configured_liveness_returns_500_when_state_not_alive() {
        let app = create_router(
            ApplicationState {
                alive: false,
                ready: false,
            }
        );
        let server = TestServer::new(app).unwrap();

        let response = server.get(&"/is_alive".to_string()).await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn configured_readiness_returns_200_when_state_alive() {
        let app = create_router(
            ApplicationState {
                alive: false,
                ready: true,
            }
        );
        let server = TestServer::new(app).unwrap();

        let response = server.get(&"/is_ready".to_string()).await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn configured_readiness_returns_500_when_state_not_alive() {
        let app = create_router(
            ApplicationState {
                alive: false,
                ready: false,
            }
        );
        let server = TestServer::new(app).unwrap();

        let response = server.get(&"/is_ready".to_string()).await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

    }

    #[tokio::test]
    async fn configured_my_car_returns_200_and_colour_black_in_json_object() {
        let routes = create_router(
            ApplicationState {
                alive: true,
                ready: false,
            }
        );
        let server = TestServer::new(routes).unwrap();

        let response: TestResponse = server.get(&"/my_car".to_string()).await;
        let response_status_code: StatusCode = response.status_code();
        let my_car: Car = serde_json::from_str(response.text().as_str()).unwrap();

        assert_eq!(response_status_code.clone(), StatusCode::OK);
        assert_eq!(my_car.clone().color, "Black".to_string());

    }
}