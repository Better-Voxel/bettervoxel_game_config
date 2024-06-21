use std::collections::HashMap;
use bevy_math::Vec3;
use bevy_render::color::Color;

use bevy_transform::prelude::Transform;
use serde::{Deserialize, Serialize};

use crate::dto::TypeDTO;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameElementDTO {
    pub name: String,
    pub value: GameElementTypeDTO,
    pub children: Option<Vec<GameElementDTO>>,
    #[serde(skip_deserializing)]
    pub attributes: Option<HashMap<String, TypeDTO>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "PascalCase", content = "value")]
pub enum GameElementTypeDTO {
    Part(PartDTO),
    Folder(FolderDTO),
    Script(ScriptDTO),
    PlayerPrefab(PlayerPrefabDTO)
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PartDTO {
    #[serde(flatten)]
    pub transform: Transform,
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderDTO {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptDTO {
    pub script: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PlayerPrefabDTO {
    pub position: Vec3,
    pub hauteur: f32,
    pub rayon: f32,
    pub camera_offset: Vec3,
    pub camera_look_at: Vec3,
}

#[cfg(test)]
mod tests {
    use bevy_math::{Quat, Vec3};
    use bevy_render::color::Color;
    use bevy_transform::prelude::Transform;
    use crate::dto::hierarchy_dto::{PartDTO, PlayerPrefabDTO};

    const PART: PartDTO = PartDTO {
        transform: Transform {
            translation: Vec3::new(1., 2., 3.),
            scale: Vec3::new(1., -1., 0.5),
            rotation: Quat::IDENTITY
        },
        color: Color::DARK_GRAY
    };

    const PART_JSON: &'static str = r#"
            {
                "translation": [1, 2, 3],
                "scale": [1, -1, 0.5],
                "rotation": [0, 0, 0, 1],
                "color": {
                    "Rgba": {
                        "red": 0.25,
                        "green": 0.25,
                        "blue": 0.25,
                        "alpha": 1.0
                    }
                }
            }"#;

    #[test]
    fn test_deserialize() {
        let part: PartDTO = serde_json::from_str(PART_JSON).unwrap();
        assert_eq!(PART, part);
    }

    const PLAYER: PlayerPrefabDTO = PlayerPrefabDTO {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        hauteur: 1.8,
        rayon: 1.0,
        camera_offset: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        camera_look_at: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    const PLAYER_JSON: & str = r#"{"position":[0.0,0.0,0.0],"hauteur":1.8,"rayon":1.0,"camera_offset":[0.0,0.0,0.0],"camera_look_at":[0.0,0.0,0.0]}"#;
    #[test]
    fn test_serialize_player_prefab() {
        let player_json = serde_json::to_string(&PLAYER).unwrap();
        println!("{player_json}");
        assert_eq!(PLAYER_JSON, player_json);
    }

    #[test]
    fn test_deserialize_player_prefab() {
        let player = serde_json::from_str(&PLAYER_JSON).unwrap();
        assert_eq!(PLAYER, player);
    }
}