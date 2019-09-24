use amethyst::{
    assets::{
        AssetStorage, Completion, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat,
    },
    controls::HideCursor,
    core::math::Vector3,
    core::transform::Transform,
    ecs::{
        Entity,
        storage::DenseVecStorage, Component, Entities, Join, ReadStorage, World, Write,
        WriteStorage,
    },
    input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    winit::{MouseButton, VirtualKeyCode},
};

use crate::components::Cube;
use crate::components::FlyCamera;
use crate::components::Grid;
use crate::components::Player;



#[derive(Default)]
pub struct Tron {
    progress: Option<ProgressCounter>,
    entity: Option<Entity>,
}

impl SimpleState for Tron {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Cube>();

        Player::init(world);
        FlyCamera::init(world);
        Grid::init(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        SimpleTrans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;

        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_mouse_button_down(&event, MouseButton::Left) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = true;
            }
        }
        Trans::None
    }
}
