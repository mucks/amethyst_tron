use crate::components::Cube;
use crate::components::Player;
use crate::prefabs::GltfScenePrefabData;
use amethyst::{
    assets::{Completion, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    core::math::Vector3,
    core::transform::Transform,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};



#[derive(Default)]
pub struct TrailSystem {}

impl<'s> System<'s> for TrailSystem {
    type SystemData = (
        WriteStorage<'s, Cube>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        PrefabLoader<'s, GltfScenePrefabData>,
        WriteStorage<'s, Handle<Prefab<GltfScenePrefabData>>>,
        Write<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut cubes, mut transforms, mut players, loader, mut handles, _lazy_update, entities): Self::SystemData,
    ) {
        for player in (&mut players).join() {
            for (pos, has_cube) in player.direction_change_positions.iter_mut() {
                if !*has_cube {
                    let mut cube = Cube::new(player.move_count, pos.clone());
                    let mut progress = ProgressCounter::default();
                    cube.handle = Some(loader.load("prefab/cube.ron", RonFormat, &mut progress));
                    let mut transform = Transform::default();
                    transform.set_translation(cube.pos);
                    transform.set_scale(Vector3::new(0.05, 0.05, 0.05));
                    entities
                        .build_entity()
                        .with(transform, &mut transforms)
                        .with(cube.handle.clone().unwrap(), &mut handles)
                        .with(cube.clone(), &mut cubes)
                        .build();

                    *has_cube = true;
                }
            }
        }
    }
}
