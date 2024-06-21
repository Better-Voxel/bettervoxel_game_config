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
    PlayerPrefab(PlayerPrefabDTO),
    Light(LightDTO)
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

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct LightDTO {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: bool,
    pub shadow_depth_bias: f32,
    pub shadow_normal_bias: f32,
    pub outer_angle: f32,
    pub inner_angle: f32,
}

#[cfg(test)]
mod tests {
    use bevy_math::{Quat, Vec3};
    use bevy_render::color::Color;
    use bevy_transform::prelude::Transform;
    use crate::dto::hierarchy_dto::{LightDTO, PartDTO, PlayerPrefabDTO};

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
        assert_eq!(PLAYER_JSON, player_json);
    }

    #[test]
    fn test_deserialize_player_prefab() {
        let player = serde_json::from_str(&PLAYER_JSON).unwrap();
        assert_eq!(PLAYER, player);
    }

    const LIGHT: LightDTO = LightDTO {
        color: Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        },
        intensity: 0.0,
        range: 0.0,
        radius: 0.0,
        shadows_enabled: false,
        shadow_depth_bias: 0.0,
        shadow_normal_bias: 0.0,
        outer_angle: 0.0,
        inner_angle: 0.0,
    };
    const LIGHT_JSON: &'static str = r#"{"color":{"Rgba":{"red":0.0,"green":0.0,"blue":0.0,"alpha":0.0}},"intensity":0.0,"range":0.0,"radius":0.0,"shadows_enabled":false,"shadow_depth_bias":0.0,"shadow_normal_bias":0.0,"outer_angle":0.0,"inner_angle":0.0}"#;

    #[test]
    fn serialize_light() {
        let light_json = serde_json::to_string(&LIGHT).unwrap();
        assert_eq!(LIGHT_JSON, light_json);
    }

    #[test]
    fn deserialize_light() {
        let light = serde_json::from_str(LIGHT_JSON).unwrap();
        assert_eq!(LIGHT, light);
    }
}