use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeDTO {
    pub fields: HashMap<String, Type>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "PascalCase", content = "value")]
pub enum Type {
    BOOLEAN(Option<bool>),

    BYTE(Option<i8>),
    SHORT(Option<i16>),
    INT(Option<i32>),
    LONG(Option<i64>),

    FLOAT(Option<f32>),
    DOUBLE(Option<f64>),

    STRING(Option<String>),

    // ARRAY(Vec<Type>),
    /// Reference to a NAMED type from the config's type registry.
    CUSTOM(String),
    /// Inline nested fields WITH values (format v1) — lets attribute values
    /// (including nested tables) round-trip without a registry entry.
    TABLE(HashMap<String, Type>),
}
