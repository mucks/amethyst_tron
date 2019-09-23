use crate::components::Player;
use crate::enums::Direction;
use crate::enums::MoveEvent;
use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, RonFormat},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::MyPrefabData;

use crate::components::Cube;

pub struct TrailSystem;

impl<'s> System<'s> for TrailSystem {
    type SystemData = (
        WriteStorage<'s, Cube>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        PrefabLoader<'s, MyPrefabData>,
        WriteStorage<'s, Handle<Prefab<MyPrefabData>>>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut cubes, mut transforms, mut players, loader, mut handles, entities): Self::SystemData,
    ) {
        for player in (&mut players).join() {
            for (pos, has_cube) in player.direction_change_positions.iter_mut() {
                if !*has_cube {
                    let handle = loader.load("prefab/cube.ron", RonFormat, ());
                    let mut transform = Transform::default();
                    transform.set_translation(pos.clone());

                    println!("{:?}", pos);
                    println!("{}", player.move_count);

                    entities
                        .build_entity()
                        .with(handle, &mut handles)
                        .with(
                            Cube {
                                id: player.move_count,
                                position: pos.clone(),
                            },
                            &mut cubes,
                        )
                        .with(transform, &mut transforms)
                        .build();

                    *has_cube = true;
                }
            }

            for (mut cube, mut transform) in (&mut cubes, &mut transforms).join() {
                transform.set_translation(cube.position);
            }
        }
    }
}
