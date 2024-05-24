use bevy_input::prelude::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindDTO {
    pub description: Option<String>,
    pub key: KeyCode,
}
