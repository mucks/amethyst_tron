use amethyst::{
    assets::{PrefabLoader, RonFormat},
    core::components::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage},
    prelude::*,
};

use crate::MyPrefabData;

pub struct Cube {
    pub id: usize,
    pub position: Vector3<f32>,
}

impl Component for Cube {
    type Storage = DenseVecStorage<Self>;
}

impl Cube {
    pub fn init(world: &mut World, position: Vector3<f32>) {
        let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/cube.ron", RonFormat, ())
        });
        let mut transform = Transform::default();
        transform.set_translation(position);

        world
            .create_entity()
            .with(handle)
            .with(Cube {id: 0 , position: Vector3::new(0.0, 0.0, 0.0)})
            .with(transform)
            .build();
    }
}
