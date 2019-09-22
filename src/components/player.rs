use crate::MyPrefabData;
use amethyst::assets::{PrefabLoader, RonFormat};
use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;

pub enum MoveEvent {
    LeftClick,
    RightClick,
}

pub enum Direction {
    Forward,  //z--
    Backward, //z++
    Left,     //x--
    Right,    //x++
}

impl Direction {
    pub fn apply_move_event(&self, move_event: MoveEvent) -> Direction {
        use Direction::*;
        use MoveEvent::*;

        match &self {
            Forward => match move_event {
                LeftClick => Left,
                RightClick => Right,
            },
            Backward => match move_event {
                LeftClick => Right,
                RightClick => Left,
            },
            Left => match move_event {
                LeftClick => Forward,
                RightClick => Backward,
            },
            Right => match move_event {
                LeftClick => Backward,
                RightClick => Forward,
            },
        }
    }
}

pub struct Player {
    pub direction: Direction,
    pub left_click_lock: bool,
    pub right_click_lock: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            direction: Direction::Forward,
            left_click_lock: false,
            right_click_lock: false,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn init(world: &mut World) {
        let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/player.ron", RonFormat, ())
        });
        world
            .create_entity()
            .with(handle)
            .with(Player::new())
            .build();
    }
}
