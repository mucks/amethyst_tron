use amethyst::{
    assets::{AssetPrefab, PrefabLoader, PrefabLoaderSystem, RonFormat},
    controls::FlyControlBundle,
    core::frame_limiter::FrameRateLimitStrategy,
    core::transform::TransformBundle,
    ecs::prelude::{ReadExpect, Resources, SystemData},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderDebugLines, RenderingBundle,
    },
    utils::{application_root_dir, scene::BasicScenePrefab},
};

use std::time::Duration;

mod components;
mod enums;
mod prefabs;
mod systems;
mod tron;

use crate::tron::Tron;

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

use amethyst_gltf::{GltfSceneLoaderSystem};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let key_bindings_path = config_dir.join("input.ron");
    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
            .with_sensitivity(0.1, 0.1),
        )?
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with(
            PrefabLoaderSystem::<prefabs::GltfScenePrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with(
            GltfSceneLoaderSystem::default(),
            "gltf_loader",
            &["scene_loader"], // This is important so that entity instantiation is performed in a single frame.
        )
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderShaded3D::default()),
        )?
        .with(
            systems::PlayerMoveSystem,
            "player_move_system",
            &["input_system"],
        )
        .with(
            systems::TrailSystem::default(),
            "trail_system",
            &["player_move_system"],
        );

    let mut game = Application::build(assets_dir, Tron::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
