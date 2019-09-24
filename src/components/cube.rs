use amethyst::{
    assets::{Handle, Prefab},
    core::math::Vector3,
    ecs::{Component, DenseVecStorage},
};

use crate::prefabs::GltfScenePrefabData;

#[derive(Clone)]
pub struct Cube {
    pub handle: Option<Handle<Prefab<GltfScenePrefabData>>>,
    pub id: usize,
    pub pos: Vector3<f32>,
}

impl Cube {
    pub fn new(id: usize, pos: Vector3<f32>) -> Self {
        Self {
            handle: None,
            id: id,
            pos: pos,
        }
    }
}
impl Component for Cube {
    type Storage = DenseVecStorage<Self>;
}
