use bevy_color::Color;
use bevy_math::{IVec3, Vec3};
use std::collections::HashMap;

use crate::dto::TypeDTO;
use crate::dto::asset_dto::AssetId;
use bevy_transform::prelude::Transform;
use serde::{Deserialize, Serialize};

/// Stable identity of one hierarchy element, preserved across save/load.
/// The basis for cross-references (e.g. [`ModelDTO::primary_part`]) and for
/// editor state that must survive a reload (selection, undo targets).
pub type InstanceId = uuid::Uuid;

fn new_instance_id() -> InstanceId {
    uuid::Uuid::new_v4()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameElementDTO {
    /// v0 files carry no ids: a fresh one is generated at parse time (and
    /// persisted on the next save).
    #[serde(default = "new_instance_id")]
    pub id: InstanceId,
    pub name: String,
    pub value: GameElementTypeDTO,
    pub children: Option<Vec<GameElementDTO>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, TypeDTO>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "PascalCase", content = "value")]
pub enum GameElementTypeDTO {
    Part(PartDTO),
    Folder(FolderDTO),
    Model(ModelDTO),
    Script(ScriptDTO),
    ModuleScript(ScriptDTO),
    PlayerPrefab(PlayerPrefabDTO),
    /// A part that doubles as a character spawn point. Shares the part
    /// shape — spawn pads are visible, collidable geometry.
    SpawnLocation(PartDTO),
    /// Makes its parent (part or model subtree) clickable.
    ClickDetector(ClickDetectorDTO),
    /// An audio emitter — positional when parented to a part.
    Sound(SoundDTO),
    SpotLight(SpotLightDTO),
    PointLight(PointLightDTO),
    Prop(PropDTO),
    Structure(StructureDTO),
}

/// Deserializes via [`PartDTORaw`]: format v1 names the field `anchored`;
/// v0 files carried the INVERSE as `gravity` (gravity = !anchored).
/// `can_collide`/`transparency`/`shape` are additive v1 fields — files
/// without them load with the Roblox defaults (collidable, opaque, block).
#[derive(Serialize, Deserialize, Debug)]
#[serde(try_from = "PartDTORaw")]
#[cfg_attr(test, derive(PartialEq))]
pub struct PartDTO {
    pub size: Vec3,
    pub color: Color,
    pub position: Transform,
    pub anchored: bool,
    pub can_collide: bool,
    pub transparency: f32,
    pub shape: PartShapeDTO,
}

/// The geometric shape of a part (Roblox `PartType`). The mesh AND the
/// collider derive from it.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PartShapeDTO {
    #[default]
    Block,
    Ball,
    Cylinder,
    Wedge,
    CornerWedge,
}

fn default_can_collide() -> bool {
    true
}

#[derive(Deserialize)]
struct PartDTORaw {
    size: Vec3,
    color: Color,
    position: Transform,
    #[serde(default)]
    anchored: Option<bool>,
    #[serde(default)]
    gravity: Option<bool>,
    #[serde(default = "default_can_collide")]
    can_collide: bool,
    #[serde(default)]
    transparency: f32,
    #[serde(default)]
    shape: PartShapeDTO,
}

impl TryFrom<PartDTORaw> for PartDTO {
    type Error = String;

    fn try_from(raw: PartDTORaw) -> Result<Self, Self::Error> {
        let anchored = raw
            .anchored
            .or(raw.gravity.map(|gravity| !gravity))
            .ok_or_else(|| "part is missing the 'anchored' field".to_string())?;
        Ok(PartDTO {
            size: raw.size,
            color: raw.color,
            position: raw.position,
            anchored,
            can_collide: raw.can_collide,
            transparency: raw.transparency,
            shape: raw.shape,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderDTO {}

/// A click region on the parent object. Distances are meters, measured from
/// the clicking character.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ClickDetectorDTO {
    #[serde(default = "default_max_activation_distance")]
    pub max_activation_distance: f32,
}

fn default_max_activation_distance() -> f32 {
    10.0
}

/// An audio emitter (Roblox `Sound`): positional when parented to a part.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SoundDTO {
    pub asset_id: AssetId,
    #[serde(default = "default_sound_volume")]
    pub volume: f32,
    #[serde(default)]
    pub looped: bool,
    #[serde(default = "default_playback_speed")]
    pub playback_speed: f32,
    /// Whether the sound starts playing when the game loads.
    #[serde(default)]
    pub playing: bool,
}

fn default_sound_volume() -> f32 {
    0.5
}

fn default_playback_speed() -> f32 {
    1.0
}

/// A grouping with an optional primary part (referenced by instance id).
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ModelDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_part: Option<InstanceId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptDTO {
    pub asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PlayerPrefabDTO {
    pub height: f32,
    pub radius: f32,
    pub camera_offset: Vec3,
    pub camera_look_at: Vec3,
    pub position: Transform,
}

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
    use crate::dto::hierarchy_dto::{
        GameElementDTO, ModelDTO, PartDTO, PartShapeDTO, PlayerPrefabDTO, PointLightDTO,
        SpotLightDTO,
    };
    use bevy_color::Color;
    use bevy_color::palettes::css::AQUA;
    use bevy_math::Vec3;
    use bevy_transform::prelude::Transform;
    use serde_json::Value;

    const PART: PartDTO = PartDTO {
        size: Vec3::new(1., 3., 0.5),
        color: Color::Srgba(AQUA),
        position: Transform::from_xyz(1., 2., 3.),
        anchored: false,
        can_collide: true,
        transparency: 0.0,
        shape: PartShapeDTO::Block,
    };

    // Format v0: parts carried `gravity` = !anchored.
    const PART_JSON_V0: &str = r#"
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
                },
                "gravity": true
            }"#;

    const PART_JSON_V1: &str = r#"
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
                },
                "anchored": false
            }"#;

    #[test]
    fn v0_gravity_field_loads_inverted_into_anchored() {
        let part: PartDTO = serde_json::from_str(PART_JSON_V0).unwrap();
        assert_eq!(PART, part);
    }

    #[test]
    fn v1_anchored_field_loads() {
        let part: PartDTO = serde_json::from_str(PART_JSON_V1).unwrap();
        assert_eq!(PART, part);
    }

    #[test]
    fn part_missing_both_fields_errors() {
        let json = r#"{
            "size": [1, 1, 1],
            "color": { "Srgba": { "red": 0.0, "green": 0.0, "blue": 0.0, "alpha": 1.0 } },
            "position": {
                "translation": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0]
            }
        }"#;
        assert!(serde_json::from_str::<PartDTO>(json).is_err());
    }

    #[test]
    fn part_serializes_anchored_never_gravity() {
        let value = serde_json::to_value(PART).unwrap();
        assert_eq!(value.get("anchored"), Some(&Value::Bool(false)));
        assert!(value.get("gravity").is_none());
    }

    // The additive v1 fields default when absent (both v0 and early-v1
    // files) and round-trip when set.
    #[test]
    fn can_collide_and_transparency_default_and_round_trip() {
        let part: PartDTO = serde_json::from_str(PART_JSON_V0).unwrap();
        assert!(part.can_collide, "missing can_collide must default to true");
        assert_eq!(part.transparency, 0.0, "missing transparency must default to 0");

        let ghost = PartDTO {
            size: Vec3::ONE,
            color: Color::Srgba(AQUA),
            position: Transform::from_xyz(0., 0., 0.),
            anchored: true,
            can_collide: false,
            transparency: 0.5,
            shape: PartShapeDTO::Block,
        };
        let back: PartDTO =
            serde_json::from_str(&serde_json::to_string(&ghost).unwrap()).unwrap();
        assert_eq!(ghost, back);
    }

    // The additive v1 shape field defaults to Block and round-trips.
    #[test]
    fn shape_defaults_to_block_and_round_trips() {
        let part: PartDTO = serde_json::from_str(PART_JSON_V0).unwrap();
        assert_eq!(part.shape, PartShapeDTO::Block, "missing shape must default");

        let ball = PartDTO {
            size: Vec3::splat(2.0),
            color: Color::Srgba(AQUA),
            position: Transform::from_xyz(0., 0., 0.),
            anchored: false,
            can_collide: true,
            transparency: 0.0,
            shape: PartShapeDTO::Ball,
        };
        let json = serde_json::to_value(&ball).unwrap();
        assert_eq!(json["shape"], Value::String("Ball".to_string()));
        let back: PartDTO = serde_json::from_value(json).unwrap();
        assert_eq!(ball, back);
    }

    #[test]
    fn elements_without_id_get_one_and_explicit_ids_survive() {
        let json = r#"{
            "name": "Thing",
            "value": { "type": "Folder", "value": {} },
            "children": null
        }"#;
        let element: GameElementDTO = serde_json::from_str(json).unwrap();
        assert!(!element.id.is_nil(), "v0 elements must receive a fresh id");

        let round_tripped: GameElementDTO =
            serde_json::from_str(&serde_json::to_string(&element).unwrap()).unwrap();
        assert_eq!(element.id, round_tripped.id, "ids must persist");
    }

    #[test]
    fn attributes_round_trip() {
        use crate::dto::type_dto::Type;
        use std::collections::HashMap;

        let json = r#"{
            "name": "Configured",
            "value": { "type": "Folder", "value": {} },
            "children": null,
            "attributes": {
                "stats": { "fields": {
                    "level": { "type": "INT", "value": 5 },
                    "nested": { "type": "TABLE", "value": {
                        "brave": { "type": "BOOLEAN", "value": true }
                    } }
                } }
            }
        }"#;
        let element: GameElementDTO = serde_json::from_str(json).unwrap();
        let attributes = element.attributes.as_ref().expect("attributes must load");
        let stats = &attributes["stats"];
        assert!(matches!(stats.fields["level"], Type::INT(Some(5))));
        let Type::TABLE(nested) = &stats.fields["nested"] else {
            panic!("inline table values must load");
        };
        assert!(matches!(nested["brave"], Type::BOOLEAN(Some(true))));
        let _: HashMap<String, Type> = nested.clone();

        // Symmetric: what serializes out parses back identically.
        let out = serde_json::to_value(&element).unwrap();
        let back: GameElementDTO = serde_json::from_value(out.clone()).unwrap();
        assert_eq!(out, serde_json::to_value(&back).unwrap());
    }

    // Format v2: SpawnLocation is a part-shaped element.
    #[test]
    fn spawn_location_round_trips_as_a_part_shape() {
        let element = GameElementDTO {
            id: uuid::Uuid::new_v4(),
            name: "Spawn".to_string(),
            value: crate::dto::hierarchy_dto::GameElementTypeDTO::SpawnLocation(PartDTO {
                size: Vec3::new(4.0, 0.5, 4.0),
                color: Color::Srgba(AQUA),
                position: Transform::from_xyz(0., 0.25, 0.),
                anchored: true,
                can_collide: true,
                transparency: 0.0,
                shape: PartShapeDTO::Block,
            }),
            children: None,
            attributes: None,
        };
        let json = serde_json::to_value(&element).unwrap();
        assert_eq!(
            json["value"]["type"],
            serde_json::Value::String("SpawnLocation".to_string())
        );
        let back: GameElementDTO = serde_json::from_value(json).unwrap();
        match back.value {
            crate::dto::hierarchy_dto::GameElementTypeDTO::SpawnLocation(part) => {
                assert!(part.anchored);
                assert_eq!(part.size, Vec3::new(4.0, 0.5, 4.0));
            }
            other => panic!("expected SpawnLocation, got {other:?}"),
        }
    }

    #[test]
    fn click_detector_defaults_and_round_trips() {
        use crate::dto::hierarchy_dto::ClickDetectorDTO;
        let detector: ClickDetectorDTO = serde_json::from_str("{}").unwrap();
        assert_eq!(detector.max_activation_distance, 10.0);

        let tuned = ClickDetectorDTO {
            max_activation_distance: 3.5,
        };
        let back: ClickDetectorDTO =
            serde_json::from_str(&serde_json::to_string(&tuned).unwrap()).unwrap();
        assert_eq!(tuned, back);
    }

    #[test]
    fn sound_defaults_and_round_trips() {
        use crate::dto::hierarchy_dto::SoundDTO;
        let minimal: SoundDTO =
            serde_json::from_str(r#"{ "asset_id": { "type": "Local", "value": "boom.ogg" } }"#)
                .unwrap();
        assert_eq!(minimal.volume, 0.5);
        assert!(!minimal.looped);
        assert_eq!(minimal.playback_speed, 1.0);
        assert!(!minimal.playing);

        let tuned = SoundDTO {
            asset_id: crate::dto::asset_dto::AssetId::Local("music.ogg".into()),
            volume: 0.8,
            looped: true,
            playback_speed: 1.25,
            playing: true,
        };
        let back: SoundDTO =
            serde_json::from_str(&serde_json::to_string(&tuned).unwrap()).unwrap();
        assert_eq!(tuned, back);
    }

    #[test]
    fn model_and_player_prefab_round_trip() {
        let model = ModelDTO {
            primary_part: Some(uuid::Uuid::new_v4()),
        };
        let back: ModelDTO =
            serde_json::from_str(&serde_json::to_string(&model).unwrap()).unwrap();
        assert_eq!(model, back);

        let player = PlayerPrefabDTO {
            height: 1.2,
            radius: 0.4,
            camera_offset: Vec3::new(0., -5., 0.),
            camera_look_at: Vec3::NEG_Z,
            position: Transform::from_xyz(0., 3., 0.),
        };
        let back: PlayerPrefabDTO =
            serde_json::from_str(&serde_json::to_string(&player).unwrap()).unwrap();
        assert_eq!(player, back);
    }

    const SPOT_LIGHT: SpotLightDTO = SpotLightDTO {
        color: Color::Srgba(AQUA),
        intensity: 0.0,
        range: 0.0,
        radius: 0.0,
        shadows_enabled: false,
        outer_angle: 0.0,
        inner_angle: 0.0,
        position: Transform::from_xyz(1., 2., 3.),
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
        position: Transform::from_xyz(1., 2., 3.),
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
