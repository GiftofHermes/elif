use bevy::prelude::*;
use elifim::pleb;

fn main() { 
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(pleb::PlebPlugin)
        .add_systems(Startup, spawn_camera)
        .run()
}


pub fn spawn_camera(
    mut commands: Commands,
) { 
    commands.spawn(Camera2dBundle::default());
}
