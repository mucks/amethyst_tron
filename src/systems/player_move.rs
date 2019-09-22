use crate::components::player::Direction;
use crate::components::player::MoveEvent;
use crate::components::Player;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct PlayerMoveSystem;

impl<'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut players, mut transforms, input, time): Self::SystemData) {
        for (player, mut transform) in (&mut players, &mut transforms).join() {
            translate_by_direction(&player.direction, &mut transform, time.delta_seconds());

            let left_click = input.action_is_down("left_click").unwrap_or(false);
            let right_click = input.action_is_down("right_click").unwrap_or(false);

            if left_click && !player.left_click_lock {
                let direction = player.direction.apply_move_event(MoveEvent::LeftClick);
                player.direction = direction;
                player.left_click_lock = true;
            } else if !left_click {
                player.left_click_lock = false;
            }

            if right_click && !player.right_click_lock {
                let direction = player.direction.apply_move_event(MoveEvent::RightClick);
                player.direction = direction;
                player.right_click_lock = true;
            } else if !right_click {
                player.right_click_lock = false;
            }
        }
    }
}

fn translate_by_direction(direction: &Direction, transform: &mut Transform, delta: f32) {
    use Direction::*;

    let velocity = 0.5;

    match direction {
        &Forward => transform.prepend_translation_z(-velocity * delta),
        &Backward => transform.prepend_translation_z(velocity * delta),
        &Left => transform.prepend_translation_x(-velocity * delta),
        &Right => transform.prepend_translation_x(velocity * delta),
    };
}
