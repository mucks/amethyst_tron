use amethyst::{
    core::math::{Point3, Vector3},
    ecs::prelude::*,
    renderer::{
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
};

pub struct Grid;

impl Grid {
    pub fn init(world: &mut World) {
        let offset = 10_000.0;

        world.add_resource(DebugLines::new());
        world.add_resource(DebugLinesParams { line_width: 2.0 });

        let mut debug_lines_component = DebugLinesComponent::with_capacity(100);
        debug_lines_component.add_direction(
            [0.0, 0.0001, 0.0].into(),
            [0.2, 0.0, 0.0].into(),
            Srgba::new(1.0, 0.0, 0.23, 1.0),
        );
        debug_lines_component.add_direction(
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.2, 0.0].into(),
            Srgba::new(0.5, 0.85, 0.1, 1.0),
        );
        debug_lines_component.add_direction(
            [0.0, 0.0001, 0.0].into(),
            [0.0, 0.0, 0.2].into(),
            Srgba::new(0.2, 0.75, 0.93, 1.0),
        );

        let width: u32 = 1000;
        let depth: u32 = 1000;
        let main_color = Srgba::new(0.4, 0.4, 0.4, 1.0);

        // Grid lines in X-axis
        for x in 0..=width {
            let (x, width, depth) = (x as f32, width as f32, depth as f32);

            let position = Point3::new(x - width / 2.0, offset, -depth / 2.0);
            let direction = Vector3::new(0.0, 0.0, depth);

            debug_lines_component.add_direction(position, direction, main_color);

            // Sub-grid lines
            if (x - width).abs() < 0.0001 {
                for sub_x in 1..10 {
                    let sub_offset = Vector3::new((1.0 / 10.0) * sub_x as f32, -0.001, 0.0);

                    debug_lines_component.add_direction(
                        position + sub_offset,
                        direction,
                        Srgba::new(0.1, 0.1, 0.1, 0.2),
                    );
                }
            }
        }

        // Grid lines in Z-axis
        for z in 0..=depth {
            let (z, width, depth) = (z as f32, width as f32, depth as f32);

            let position = Point3::new(-width / 2.0, offset, z - depth / 2.0);
            let direction = Vector3::new(width, 0.0, 0.0);

            debug_lines_component.add_direction(position, direction, main_color);

            // Sub-grid lines
            if (z - depth).abs() < 0.0001 {
                for sub_z in 1..10 {
                    let sub_offset = Vector3::new(0.0, -0.001, (1.0 / 10.0) * sub_z as f32);

                    debug_lines_component.add_direction(
                        position + sub_offset,
                        direction,
                        Srgba::new(0.1, 0.1, 0.1, 0.2),
                    );
                }
            }
        }

        world.register::<DebugLinesComponent>();
        world.create_entity().with(debug_lines_component).build();
    }
}
