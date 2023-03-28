use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

) { 
    for i in 0..10 { 
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-150. + (i as f32) * 20., 0., 0.)),
            ..default()
            },
            Pleb {},
        )
        );
    }
    
}

pub fn spawn_camera(
    mut commands: Commands,
) { 
    commands.spawn(Camera2dBundle::default());
}
