use amethyst::{
    assets::{
        AssetPrefab, AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, PrefabLoaderSystem,
        ProgressCounter, RonFormat,
    },
    core::{
        math::Vector3,
        transform::Transform,
        {Named, Parent},
    },
    derive::PrefabData,
    ecs::{
        storage::DenseVecStorage, Component, Entities, Entity, Join, ReadStorage, World,
        WriteStorage,
    },
    prelude::*,
    utils::application_root_dir,
    Error,
};

use serde::{Deserialize, Serialize};
use amethyst_gltf::{GltfPrefab, GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystem};

#[derive(Deserialize, Serialize, PrefabData)]
pub struct GltfScenePrefabData {
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
}

impl Component for GltfScenePrefabData {
    type Storage = DenseVecStorage<Self>;
}