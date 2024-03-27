mod materials;
mod world;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use materials::material::Material;
use materials::sand::Sand;

use crate::world::grid::Grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(300., 300.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let _ = Sand::new((0., 0.), commands, meshes, materials);
}

fn update(mut query: Query<(&mut Sand, &mut Transform)>) {
    let (mut s, mut transform) = query.single_mut();
    transform.translation.x = s.x as f32;
    transform.translation.y = s.y as f32;

    let grid = Grid::new(0, 0);
    s.interact(&grid);
}
