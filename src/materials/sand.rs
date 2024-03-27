use crate::world::grid::Grid;

use super::material::Material;
use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::primitives::Rectangle,
    prelude::default,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
};

#[derive(Component)]
pub struct Sand {
    pub x: f32,
    pub y: f32,
    dir: u8,
}

impl Sand {
    pub fn new(
        pos: (f32, f32),
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let shape = Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0)));
        let color = Color::rgba(1., 0., 0., 1.);

        let s = Self {
            x: (-150. + 5. + pos.0 as f32),
            y: (150. - 5. - pos.1 as f32),
            dir: 0,
        };
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(
                    -150. + 5. + pos.0 as f32,
                    150. - 5. - pos.1 as f32,
                    0.0,
                ),
                ..default()
            },
            Self {
                x: (-150. + 5. + pos.0 as f32),
                y: (150. - 5. - pos.1 as f32),
                dir: 0,
            },
        ));

        s
    }

    pub fn move_to(&mut self, new_pos: (f32, f32)) {
        self.x = new_pos.0;
        self.y = new_pos.1;
    }
}

impl Material for Sand {
    fn render(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    }

    fn to_char(&self) -> char {
        'S'
    }

    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn interact(&mut self, _: &Grid) {
        if self.y >= -140. {
            self.y -= 10.;
        }
    }
}
