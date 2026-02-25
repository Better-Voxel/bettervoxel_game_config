use bevy_input::keyboard::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindDTO {
    pub description: Option<String>,
    pub key: KeyCode,
}
