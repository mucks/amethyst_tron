use amethyst::{
    assets::{PrefabLoader, RonFormat},
    core::components::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
};

use crate::enums::Direction;
use crate::MyPrefabData;

pub struct Player {
    pub direction: Direction,
    pub left_click_lock: bool,
    pub right_click_lock: bool,
    pub direction_change_positions: Vec<(Vector3<f32>, bool)>,
    pub move_count: usize,
}

impl Player {
    pub fn new() -> Player {
        Player {
            direction: Direction::Forward,
            left_click_lock: false,
            right_click_lock: false,
            direction_change_positions: Vec::new(),
            move_count: 0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn init(world: &mut World) -> Entity {
        let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/player.ron", RonFormat, ())
        });

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.2, 0.0);

        let mut player = Player::new();
        player
            .direction_change_positions
            .push((transform.translation().clone(), false));

        world
            .create_entity()
            .with(handle)
            .with(player)
            .with(transform)
            .build()
    }
}
