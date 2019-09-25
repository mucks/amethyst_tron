use amethyst::{
    assets::ProgressCounter,
    controls::HideCursor,
    input::{is_key_down, is_mouse_button_down},
    prelude::*,
    winit::{MouseButton, VirtualKeyCode},
};

use crate::components::Cube;
use crate::components::FlyCamera;
use crate::components::Grid;
use crate::components::Player;
use crate::components::Trail;

use crate::prefabs::gltf_prefabs::{init_prefabs, update_prefabs};

#[derive(Default)]
pub struct Tron {
    prefab_loading_progress: Option<ProgressCounter>,
    player_exists: bool,
}

impl SimpleState for Tron {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Cube>();

        self.player_exists = false;
        self.prefab_loading_progress = Some(init_prefabs(world));

        Trail::draw(world);
        FlyCamera::init(world);
        Grid::init(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref counter) = self.prefab_loading_progress.as_ref() {
            if counter.is_complete() {
                self.prefab_loading_progress = None;
                update_prefabs(&mut data.world);

                if !self.player_exists {
                    Player::init(&mut data.world);
                    self.player_exists = true;
                }
            }
        }
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
