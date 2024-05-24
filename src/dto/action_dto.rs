use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionDTO {
    #[serde(rename = "type")]
    pub data_type: Option<String>,
    pub side: ActionSide,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ActionSide {
    Client,
    Server,
    Both,
    Debug,
}