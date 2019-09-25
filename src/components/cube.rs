use amethyst::{
    core::math::Vector3,
    ecs::{Component, DenseVecStorage},
};


#[derive(Clone)]
pub struct Cube {
    pub id: usize,
    pub pos: Vector3<f32>,
}

impl Cube {
    pub fn new(id: usize, pos: Vector3<f32>) -> Self {
        Self {
            id: id,
            pos: pos,
        }
    }
}
impl Component for Cube {
    type Storage = DenseVecStorage<Self>;
}
