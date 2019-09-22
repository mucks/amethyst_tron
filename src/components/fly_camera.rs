use amethyst::prelude::*;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::assets::{PrefabLoader, RonFormat};
use crate::MyPrefabData;

pub struct FlyCamera;

impl Component for FlyCamera {
    type Storage = DenseVecStorage<Self>;
}

impl FlyCamera {
    pub fn init(world: &mut World) {
        let camera_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/fly_camera.ron", RonFormat, ())
        });
        world
            .create_entity()
            .named("Fly Camera")
            .with(camera_handle)
            .build();
    }
}
