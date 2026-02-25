use bson::Uuid;
use bson::oid::ObjectId;
use bson::serde_helpers::object_id::AsHexString;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "PascalCase", content = "value")]
#[cfg_attr(test, derive(PartialEq))]
pub enum AssetId {
    PublicCloud(Uuid),
    PrivateCloud {
        #[serde(with = "AsHexString")]
        asset_id: ObjectId,
        #[serde(with = "AsHexString")]
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
