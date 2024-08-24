use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TerrainDTO {
    pub skybox: Option<SkyboxDTO>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyboxDTO {
    pub cube_map: CubeMapDTO,
    pub environment_map_light: Option<EnvironmentMapLightDTO>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CubeMapDTO {
    pub texture: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentMapLightDTO {
    pub specular: String,
    pub diffuse: String,
}