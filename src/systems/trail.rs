use crate::components::Cube;
use crate::components::Player;
use crate::prefabs::gltf_prefabs::GltfScenePrefabs;
use amethyst::{core::math::Vector3, core::transform::Transform, ecs::prelude::*};

use amethyst::renderer::{rendy::mesh::MeshBuilder, rendy::mesh::Position};

#[derive(Default)]
pub struct TrailSystem {}

impl<'s> System<'s> for TrailSystem {
    type SystemData = (
        Read<'s, GltfScenePrefabs>,
        WriteStorage<'s, Cube>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Write<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (prefabs, mut cubes, mut players, mut transforms, lazy_update, entities): Self::SystemData,
    ) {
        for player in (&mut players).join() {
            for (pos, has_cube) in player.direction_change_positions.iter_mut() {
                if !*has_cube {
                    if let Some(cube_prefab) = prefabs.get_prefab("cube") {
                        let cube = Cube::new(player.move_count, pos.clone());
                        let mut transform = Transform::default();
                        transform.set_translation(cube.pos);
                        transform.set_scale(Vector3::new(0.05, 0.05, 0.05));

                        let entity = entities
                            .build_entity()
                            .with(cube, &mut cubes)
                            .with(transform, &mut transforms)
                            .build();

                        lazy_update.insert(entity, cube_prefab.clone());
                    }

                    *has_cube = true;
                }
            }
        }
    }
}
