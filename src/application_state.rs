use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    pub alive: bool,
    pub ready: bool,
}