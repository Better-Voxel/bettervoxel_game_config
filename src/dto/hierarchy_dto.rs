use bevy_color::Color;
use bevy_math::{IVec3, Vec3};
use std::collections::HashMap;

use crate::dto::asset_dto::AssetId;
use crate::dto::TypeDTO;
use bevy_transform::prelude::Transform;
use serde::{Deserialize, Serialize};

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
    // PlayerPrefab(PlayerPrefabDTO),
    SpotLight(SpotLightDTO),
    PointLight(PointLightDTO),
    Prop(PropDTO),
    Structure(StructureDTO),
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PartDTO {
    pub size: Vec3,
    pub color: Color,
    pub position: Transform,
    pub gravity: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderDTO {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptDTO {
    pub asset_id: AssetId,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[cfg_attr(test, derive(PartialEq))]
// pub struct PlayerPrefabDTO {
//     pub height: f32,
//     pub radius: f32,
//     pub camera_offset: Vec3,
//     pub camera_look_at: Vec3,
// }

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SpotLightDTO {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: bool,
    pub outer_angle: f32,
    pub inner_angle: f32,
    pub position: Transform,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PointLightDTO {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: bool,
    pub position: Transform,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PropDTO {
    pub asset_id: AssetId,
    pub position: IVec3,
    pub rotation: IVec3,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct StructureDTO {
    pub asset_id: AssetId,
    pub position: IVec3,
    pub rotation: IVec3,
}

#[cfg(test)]
mod tests {
    use crate::dto::hierarchy_dto::{PartDTO, PointLightDTO, SpotLightDTO};
    use bevy_color::palettes::css::AQUA;
    use bevy_color::Color;
    use bevy_math::Vec3;
    use bevy_transform::prelude::Transform;
    use serde_json::Value;

    const PART: PartDTO = PartDTO {
        size: Vec3::new(1., 3., 0.5),
        color: Color::Srgba(AQUA),
        position: Transform::from_xyz(1., 2., 3.),
        gravity: true,
    };

    const PART_JSON: &str = r#"
            {
                "size": [1, 3, 0.5],
                "color": {
                    "Srgba": {
                        "red": 0.0,
                        "green": 1.0,
                        "blue": 1.0,
                        "alpha": 1.0
                    }
                },
                "position": {
                    "translation": [1.0, 2.0, 3.0],
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0]
                }
            }"#;

    #[test]
    fn test_deserialize() {
        let part: PartDTO = serde_json::from_str(PART_JSON).unwrap();
        assert_eq!(PART, part);
    }

    // const PLAYER: PlayerPrefabDTO = PlayerPrefabDTO {
    //     height: 2.0,
    //     radius: 1.0,
    //     camera_offset: Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     camera_look_at: Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    // };

    // const PLAYER_JSON: &str = r#"{
    //     "height": 2.0,
    //     "radius": 1.0,
    //     "camera_offset": [0.0,0.0,0.0],
    //     "camera_look_at": [0.0,0.0,0.0]
    // }"#;
    // #[test]
    // fn test_serialize_player_prefab() {
    //     let player_prefab = serde_json::to_value(&PLAYER).unwrap();
    //     let player_prefab_json: Value = serde_json::from_str(PLAYER_JSON).unwrap();
    //     println!("{:?}", player_prefab);
    //     assert_eq!(player_prefab_json, player_prefab);
    // }
    //
    // #[test]
    // fn test_deserialize_player_prefab() {
    //     let player = serde_json::from_str(&PLAYER_JSON).unwrap();
    //     assert_eq!(PLAYER, player);
    // }

    const SPOT_LIGHT: SpotLightDTO = SpotLightDTO {
        color: Color::Srgba(AQUA),
        intensity: 0.0,
        range: 0.0,
        radius: 0.0,
        shadows_enabled: false,
        outer_angle: 0.0,
        inner_angle: 0.0,
        position: Transform::from_xyz(1., 2., 3.)
    };
    const SPOT_LIGHT_JSON: &str = r#"{
        "color": {
            "Srgba": {
                "red": 0.0,
                "green": 1.0,
                "blue": 1.0,
                "alpha": 1.0
            }
        },
        "intensity": 0.0,
        "range": 0.0,
        "radius": 0.0,
        "shadows_enabled": false,
        "outer_angle": 0.0,
        "inner_angle": 0.0,
        "position": {
            "translation": [1.0, 2.0, 3.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0]
        }
    }"#;

    #[test]
    fn serialize_spot_light() {
        let spot_light = serde_json::to_value(&SPOT_LIGHT).unwrap();
        let spot_light_json: Value = serde_json::from_str(SPOT_LIGHT_JSON).unwrap();
        println!("{:?}", spot_light);
        assert_eq!(spot_light_json, spot_light);
    }

    #[test]
    fn deserialize_spot_light() {
        let spot_light = serde_json::from_str(SPOT_LIGHT_JSON).unwrap();
        assert_eq!(SPOT_LIGHT, spot_light);
    }

    const POINT_LIGHT: PointLightDTO = PointLightDTO {
        color: Color::Srgba(AQUA),
        intensity: 0.0,
        range: 0.0,
        radius: 0.0,
        shadows_enabled: false,
        position: Transform::from_xyz(1., 2., 3.)
    };
    const POINT_LIGHT_JSON: &str = r#"{
        "color": {
            "Srgba": {
                "red": 0.0,
                "green": 1.0,
                "blue": 1.0,
                "alpha": 1.0
            }
        },
        "intensity": 0.0,
        "range": 0.0,
        "radius": 0.0,
        "shadows_enabled": false,
        "position": {
            "translation": [1.0, 2.0, 3.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0]
        }
    }"#;

    #[test]
    fn serialize_point_light() {
        let point_light = serde_json::to_value(&POINT_LIGHT).unwrap();
        let point_light_json: Value = serde_json::from_str(POINT_LIGHT_JSON).unwrap();
        println!("{:?}", point_light);
        assert_eq!(point_light_json, point_light);
    }

    #[test]
    fn deserialize_point_light() {
        let point_light = serde_json::from_str(POINT_LIGHT_JSON).unwrap();
        assert_eq!(POINT_LIGHT, point_light);
    }
}