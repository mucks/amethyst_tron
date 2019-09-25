use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{
        storage::DenseVecStorage, Component, Entities, LazyUpdate, Read, World, Write, WriteStorage,
    },
};

use crate::enums::Direction;
use crate::prefabs::gltf_prefabs::GltfScenePrefabs;

#[derive(Default)]
pub struct Player {
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
    pub fn init(world: &mut World) {
        world.exec(
            |(prefabs, mut transforms, mut players, lazy_update, entities): (
                Read<'_, GltfScenePrefabs>,
                WriteStorage<'_, Transform>,
                WriteStorage<'_, Player>,
                Write<'_, LazyUpdate>,
                Entities<'_>,
            )| {
                let player = Player::default();

                let mut transform = Transform::default();
                transform.set_translation_xyz(0.0, 10000.2, 0.0);
                transform.set_scale(Vector3::new(0.2, 0.2, 0.2));

                let entity = entities
                    .build_entity()
                    .with(transform, &mut transforms)
                    .with(player, &mut players)
                    .build();

                if let Some(player_prefab) = prefabs.get_prefab("player") {
                    lazy_update.insert(entity, player_prefab.clone());
                }
            },
        )
    }
}
