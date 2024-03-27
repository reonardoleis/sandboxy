use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    render::mesh::Mesh,
    sprite::ColorMaterial,
};

use crate::world::grid::Grid;

pub trait Material {
    fn render(
        &self,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
    );
    fn to_char(&self) -> char;
    fn pos(&self) -> (f32, f32);
    fn interact(&mut self, grid: &Grid);
}
