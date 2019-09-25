pub struct Trail;
use amethyst::{
    assets::AssetLoaderSystemData,
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{
        palette::{LinSrgba, Srgb, Srgba},
        rendy::{
            mesh::{MeshBuilder, Normal, Position, TexCoord},
            texture::palette::load_from_linear_rgba,
        },
        shape::Shape,
        types::{Mesh, Texture},
        Material, MaterialDefaults,
    },
};

impl Trail {
    pub fn draw(world: &mut World) {
        let my_cube = Shape::Cube.generate::<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>(None);
        let vertices = vec![
            //top (0, 0, 1)
            Position([-1.0, -1.0, 1.0]),
            Position([1.0, -1.0, 1.0]),
            Position([1.0, 1.0, 1.0]),
            Position([-1.0, 1.0, 1.0]),
            //bottom (0, 0, -1.0)
            Position([1.0, 1.0, -1.0]),
            Position([-1.0, 1.0, -1.0]),
            Position([-1.0, -1.0, -1.0]),
            Position([1.0, -1.0, -1.0]),
            //right (1.0, 0, 0)
            Position([1.0, -1.0, -1.0]),
            Position([1.0, 1.0, -1.0]),
            Position([1.0, 1.0, 1.0]),
            Position([1.0, -1.0, 1.0]),
            //left (-1.0, 0, 0)
            Position([-1.0, 1.0, 1.0]),
            Position([-1.0, -1.0, 1.0]),
            Position([-1.0, -1.0, -1.0]),
            Position([-1.0, 1.0, -1.0]),
            //front (0, 1.0, 0)
            Position([-1.0, 1.0, -1.0]),
            Position([1.0, 1.0, -1.0]),
            Position([1.0, 1.0, 1.0]),
            Position([-1.0, 1.0, 1.0]),
            //back (0, -1.0, 0)
            Position([1.0, -1.0, 1.0]),
            Position([-1.0, -1.0, 1.0]),
            Position([-1.0, -1.0, -1.0]),
            Position([1.0, -1.0, -1.0]),
        ];

        let normals = vec![
            Normal([0.0, -1.0, 0.0]),
            Normal([0.0, 1.0, 0.0]),
            Normal([-1.0, 0.0, 0.0]),
            Normal([1.0, 0.0, 0.0]),
            Normal([0.0, 0.0, 1.0]),
            Normal([0.0, 0.0, -1.0]),
        ];

        let tex_coords = vec![
            TexCoord([0.0, 0.0]),
            TexCoord([1.0, 0.0]),
            TexCoord([1.0, 1.0]),
            TexCoord([0.0, 1.0]),
            TexCoord([0.0, 0.0]),
            TexCoord([1.0, 0.0]),
            TexCoord([1.0, 1.0]),
            TexCoord([0.0, 1.0]),
            TexCoord([0.0, 0.0]),
            TexCoord([1.0, 0.0]),
            TexCoord([1.0, 1.0]),
            TexCoord([0.0, 1.0]),
            TexCoord([0.0, 0.0]),
            TexCoord([1.0, 0.0]),
            TexCoord([1.0, 1.0]),
            TexCoord([0.0, 1.0]),
            TexCoord([0.0, 0.0]),
            TexCoord([1.0, 0.0]),
            TexCoord([1.0, 1.0]),
            TexCoord([0.0, 1.0]),
            TexCoord([0.0, 0.0]),
            TexCoord([1.0, 0.0]),
            TexCoord([1.0, 1.0]),
            TexCoord([0.0, 1.0]),
        ];

        let indices: &[u16] = &[
            0, 1, 2, 2, 3, 0, // top
            4, 6, 5, 6, 4, 7, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 14, 13, 14, 12, 15, // left
            16, 18, 17, 18, 16, 19, // front
            20, 21, 22, 22, 23, 20, // back
        ];

        let mesh_builder = MeshBuilder::new()
            .with_vertices(vertices)
            .with_vertices(normals)
            .with_vertices(tex_coords)
            .with_indices(indices);

        let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(my_cube.into(), ())
        });

        let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
                (),
            )
        });

        let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

        let mtl = world.exec(|mtl_loader: AssetLoaderSystemData<'_, Material>| {
            mtl_loader.load_from_data(
                Material {
                    albedo: albedo.clone(),
                    ..mat_defaults.clone()
                },
                (),
            )
        });

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 10000.0, 0.0);

        world
            .create_entity()
            .with(mesh)
            .with(mtl)
            .with(transform)
            .build();
    }
}
