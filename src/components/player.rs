use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, RonFormat},
    core::{math::Vector3, transform::Transform},
    ecs::{
        storage::DenseVecStorage, Component, Entities, Entity, World,
        WriteStorage,
    },
};

use crate::enums::Direction;
use crate::prefabs::GltfScenePrefabData;

#[derive(Default)]
pub struct Player {
    pub handle: Option<Handle<Prefab<GltfScenePrefabData>>>,
    pub direction: Direction,
    pub left_click_lock: bool,
    pub right_click_lock: bool,
    pub direction_change_positions: Vec<(Vector3<f32>, bool)>,
    pub move_count: usize,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn init(world: &mut World) -> Entity {
        world.exec(
            |(loader, mut player_prefabs, mut players, mut transforms, entities): (
                PrefabLoader<'_, GltfScenePrefabData>,
                WriteStorage<'_, Handle<Prefab<GltfScenePrefabData>>>,
                WriteStorage<'_, Player>,
                WriteStorage<'_, Transform>,
                Entities<'_>,
            )| {
                let mut player = Player::default();
                player.handle = Some(loader.load("prefab/player.ron", RonFormat, ()));

                let mut transform = Transform::default();
                transform.set_translation_xyz(5.0, 0.2, 5.0);
                transform.set_scale(Vector3::new(0.2, 0.2, 0.2));

                entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(player.handle.clone().unwrap(), &mut player_prefabs)
                    .with(player, &mut players)
                    .build()
            },
        )
    }
}
