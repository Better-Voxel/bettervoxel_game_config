use bson::serde_helpers::serialize_object_id_as_hex_string;
use std::path::PathBuf;
use bson::oid::ObjectId;
use bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "PascalCase", content = "value")]
#[cfg_attr(test, derive(PartialEq))]
pub enum AssetId {
    PublicCloud(Uuid),
    PrivateCloud {
        #[serde(serialize_with = "serialize_object_id_as_hex_string")]
        asset_id: ObjectId,
        #[serde(serialize_with = "serialize_object_id_as_hex_string")]
        version_id: ObjectId,
    },
    Local(PathBuf),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetDTO {
    pub id: AssetId,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetType {
    Image,
    Script,
    Props,
    Structure,
    Sound,
    Music,
    // Shader,
}