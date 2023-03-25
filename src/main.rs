use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() { 
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_plebs)
        .add_startup_system(spawn_camera)
        .run()
}


#[derive(Component)]
pub struct Pleb {}


pub fn spawn_plebs(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) { 
    let window = window_query.get_single().expect("Bevy should provide a main window");

    commands.spawn(
        (SpriteBundle { 
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
            texture: asset_server.load("sprites/pleb.png"),
            ..default()
        }, 
        Pleb {},
    )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) { 
    let window = window_query.get_single().expect("Bevy should provide a main window");

    commands.spawn(
    Camera2dBundle {     
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
        ..default()
        }
    );
}
