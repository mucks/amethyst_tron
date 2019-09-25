use amethyst::{
    assets::{AssetPrefab, PrefabData, ProgressCounter},
    core::Named,
    derive::PrefabData,
    ecs::{storage::DenseVecStorage, Component, Entity},
    Error,
};

use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PrefabData)]
pub struct GltfScenePrefabData {
    pub name: Option<Named>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
}

impl Component for GltfScenePrefabData {
    type Storage = DenseVecStorage<Self>;
}
