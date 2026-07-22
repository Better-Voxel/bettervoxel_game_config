use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dto::hierarchy_dto::GameElementDTO;
use crate::dto::lighting_dto::LightingDTO;
use crate::dto::terrain_dto::TerrainDTO;
use crate::dto::type_dto::Type;
use crate::dto::{ActionDTO, AssetDTO, KeyBindDTO, TypeDTO};

/// The version this crate WRITES. Loaders accept any version up to it:
/// older shapes are absorbed at parse time (serde defaults and aliases) and
/// upgraded on the next save. Files from a NEWER version are refused — see
/// [`ConfigDTO::check_version`].
///
/// v0 (implicit): no `version`, no element ids, parts carried
/// `gravity = !anchored`, attributes never deserialized.
/// v1: `version` field, per-element `id`, `anchored`, symmetric attributes
/// with inline `Table` values, `Model`/`ModuleScript`/`PlayerPrefab` kinds;
/// later additions while the format is pre-release: `can_collide`/
/// `transparency` part fields, the `SpawnLocation`/`ClickDetector`/`Sound`
/// kinds, and the optional `lighting` section (files using them are
/// unreadable by older parsers — the version gates when the format is next
/// cut, not per addition).
pub const FORMAT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDTO {
    /// Save-format version; missing in v0 files (defaults to 0).
    #[serde(default)]
    pub version: u32,
    pub assets: HashMap<String, AssetDTO>,
    pub types: HashMap<String, TypeDTO>,
    pub actions: HashMap<String, ActionDTO>,
    pub keybinds: HashMap<String, KeyBindDTO>,
    pub hierarchy: Vec<GameElementDTO>,
    pub terrain: TerrainDTO,
    /// Global lighting (additive v1 section; absent = engine defaults).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lighting: Option<LightingDTO>,
}

#[derive(Debug)]
pub struct GameConfigError {
    message: String,
}

impl GameConfigError {
    pub fn with_string(message: String) -> Self {
        Self { message }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Copy, PartialEq)]
enum VisitState {
    Visiting,
    Done,
}

fn visit_type<'a>(
    name: &'a String,
    types: &'a HashMap<String, TypeDTO>,
    states: &mut HashMap<&'a String, VisitState>,
) -> Result<(), GameConfigError> {
    match states.get(name) {
        Some(VisitState::Done) => return Ok(()),
        Some(VisitState::Visiting) => {
            return Err(GameConfigError::with_string(format!(
                "Circular type reference through '{}'",
                name
            )));
        }
        None => {}
    }
    let Some(type_dto) = types.get(name) else {
        return Err(GameConfigError::with_string(format!(
            "Unknown custom type: {}",
            name
        )));
    };
    states.insert(name, VisitState::Visiting);
    visit_fields(&type_dto.fields, types, states)?;
    states.insert(name, VisitState::Done);
    Ok(())
}

fn visit_fields<'a>(
    fields: &'a HashMap<String, Type>,
    types: &'a HashMap<String, TypeDTO>,
    states: &mut HashMap<&'a String, VisitState>,
) -> Result<(), GameConfigError> {
    for field in fields.values() {
        match field {
            Type::CUSTOM(target) => visit_type(target, types, states)?,
            Type::TABLE(nested) => visit_fields(nested, types, states)?,
            _ => {}
        }
    }
    Ok(())
}

impl ConfigDTO {
    /// Errors when the file was written by a newer engine/editor than this
    /// crate understands. Older versions load transparently.
    pub fn check_version(&self) -> Result<(), GameConfigError> {
        if self.version > FORMAT_VERSION {
            return Err(GameConfigError::with_string(format!(
                "File format v{} is newer than the supported v{} — update the engine",
                self.version, FORMAT_VERSION
            )));
        }
        Ok(())
    }

    /// Validates the type registry: every referenced custom type exists and
    /// no reference chain (including through inline tables) forms a cycle.
    pub fn check_circular_types(&self) -> Result<(), GameConfigError> {
        let mut states: HashMap<&String, VisitState> = HashMap::new();
        for name in self.types.keys() {
            visit_type(name, &self.types, &mut states)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry(entries: &[(&str, &[(&str, Type)])]) -> HashMap<String, TypeDTO> {
        entries
            .iter()
            .map(|(name, fields)| {
                (
                    name.to_string(),
                    TypeDTO {
                        fields: fields
                            .iter()
                            .map(|(key, value)| (key.to_string(), value.clone()))
                            .collect(),
                    },
                )
            })
            .collect()
    }

    fn config_with_types(types: HashMap<String, TypeDTO>) -> ConfigDTO {
        ConfigDTO {
            version: FORMAT_VERSION,
            assets: HashMap::new(),
            types,
            actions: HashMap::new(),
            keybinds: HashMap::new(),
            hierarchy: Vec::new(),
            terrain: TerrainDTO { skybox: None },
            lighting: None,
        }
    }

    #[test]
    fn detects_direct_cycle() {
        let config = config_with_types(registry(&[
            ("A", &[("b", Type::CUSTOM("B".to_string()))]),
            ("B", &[("a", Type::CUSTOM("A".to_string()))]),
        ]));
        assert!(config.check_circular_types().is_err());
    }

    #[test]
    fn detects_self_cycle() {
        let config = config_with_types(registry(&[(
            "A",
            &[("me", Type::CUSTOM("A".to_string()))],
        )]));
        assert!(config.check_circular_types().is_err());
    }

    #[test]
    fn detects_cycle_through_inline_table() {
        let config = config_with_types(registry(&[(
            "A",
            &[(
                "nested",
                Type::TABLE(HashMap::from([(
                    "back".to_string(),
                    Type::CUSTOM("A".to_string()),
                )])),
            )],
        )]));
        assert!(config.check_circular_types().is_err());
    }

    #[test]
    fn accepts_diamond_dependencies() {
        // A -> B, A -> C, B -> D, C -> D: shared but acyclic.
        let config = config_with_types(registry(&[
            (
                "A",
                &[
                    ("b", Type::CUSTOM("B".to_string())),
                    ("c", Type::CUSTOM("C".to_string())),
                ],
            ),
            ("B", &[("d", Type::CUSTOM("D".to_string()))]),
            ("C", &[("d", Type::CUSTOM("D".to_string()))]),
            ("D", &[("value", Type::INT(Some(1)))]),
        ]));
        assert!(config.check_circular_types().is_ok());
    }

    #[test]
    fn rejects_unknown_type_reference() {
        let config = config_with_types(registry(&[(
            "A",
            &[("ghost", Type::CUSTOM("Nope".to_string()))],
        )]));
        assert!(config.check_circular_types().is_err());
    }

    #[test]
    fn newer_version_is_refused_and_older_accepted() {
        let mut config = config_with_types(HashMap::new());
        config.version = FORMAT_VERSION + 1;
        assert!(config.check_version().is_err());
        config.version = 0;
        assert!(config.check_version().is_ok());
    }
}
