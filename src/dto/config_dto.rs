use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::dto::hierarchy_dto::GameElementDTO;
use crate::dto::{ActionDTO, AssetDTO, KeyBindDTO, Type, TypeDTO};
use crate::dto::terrain_dto::TerrainDTO;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDTO {
    pub assets: HashMap<String, AssetDTO>,
    pub types: HashMap<String, TypeDTO>,
    pub actions: HashMap<String, ActionDTO>,
    pub keybinds: HashMap<String, KeyBindDTO>,
    pub hierarchy: Vec<GameElementDTO>,
    pub terrain: TerrainDTO,
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


impl ConfigDTO {
    pub fn check_circular_types(&self) -> Result<(), GameConfigError> {
        let mut deps: HashMap<&String, HashSet<&String>> = HashMap::new();

        for (key, type_dto) in self.types.iter() {
            let mut set: HashSet<&String> = HashSet::new();
            for (name, field) in type_dto.fields.iter() {
                if let Type::CUSTOM(value) = field {
                    if !self.types.contains_key(value) {
                        return Err(GameConfigError::with_string(format!(
                            "Unknown custom type for field {}: {}",
                            name, value
                        )));
                    }

                    if set.insert(value) {
                        if let Some(subset) = deps.get(value) {
                            if subset.contains(key) {
                                return Err(GameConfigError::with_string(format!(
                                    "Circular include: {} -> {}",
                                    value, key
                                )));
                            }
                            set.extend(subset);
                        }
                    }
                }
            }
            if !deps.is_empty() {
                deps.insert(key, set);
            }
        }
        Ok(())
    }
}
