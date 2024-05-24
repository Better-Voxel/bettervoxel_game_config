use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetDTO {
    #[serde(rename = "type")]
    pub asset_type: AssetType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AssetType {
    Image,
    Script,
    Texture,
    Model,
    Sound,
    Music,
    Shader,
}