use crate::MyPrefabData;
use amethyst::assets::{PrefabLoader, RonFormat};
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;

pub struct FlyCamera;

impl Component for FlyCamera {
    type Storage = DenseVecStorage<Self>;
}

impl FlyCamera {
    pub fn init(world: &mut World) {
        let camera_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefabs/fly_camera.ron", RonFormat, ())
        });
        world
            .create_entity()
            .named("Fly Camera")
            .with(camera_handle)
            .build();
    }
}
