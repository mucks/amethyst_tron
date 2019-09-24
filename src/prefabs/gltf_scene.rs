use amethyst::{
    assets::{
        AssetPrefab, AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, PrefabLoaderSystem,
        ProgressCounter, RonFormat,
    },
    derive::PrefabData,
    ecs::{
        storage::DenseVecStorage, Component, Entities, Entity, Join, ReadStorage, World,
        WriteStorage,
    },
    core::Named,
    Error,
};

use serde::{Deserialize, Serialize};
use amethyst_gltf::{GltfPrefab, GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystem};

#[derive(Deserialize, Serialize, PrefabData)]
pub struct GltfScenePrefabData {
    pub name: Option<Named>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
}

impl Component for GltfScenePrefabData {
    type Storage = DenseVecStorage<Self>;
}