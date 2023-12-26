use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use fixedbitset::FixedBitSet;
use crate::actions::{Action, CardinalDirection};

#[derive(Component)]
pub struct Pleb {
    gene: FixedBitSet,
    energy: u8,
    network: NeuralNetwork
}

impl Pleb  {
    pub fn new(gene: FixedBitSet) -> Pleb { 
        Pleb { 
            network: NeuralNetwork::new(&gene),
            gene: gene, 
            energy: 100,
         }
    }

    fn default() -> Pleb { 
        let gene = FixedBitSet::with_capacity(10);
        Pleb::new(gene)
    }

}

pub struct NeuralNetwork { 
    network: usize // TODO: make this an actual neural network
}

impl NeuralNetwork { 
    fn new(gene: &FixedBitSet) -> NeuralNetwork { 
        NeuralNetwork { 
            network: gene.len()
        }
    }

    fn compute(&self) -> Action { 
        // TODO: compute with an actual neural network
        Action::Walk(CardinalDirection::North)
    }
}


pub struct PlebPlugin;

impl Plugin for PlebPlugin { 
    fn build(&self, app: &mut App) { 
        app.add_systems(PostStartup, spawn_plebs)
            .add_systems(Update, kill_plebs)
            .add_systems(Update, act);
    }
}

fn spawn_plebs(
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
            Pleb::default(),
        )
        );
    }
    
}

fn kill_plebs(
    mut commands: Commands,
    query: Query<(Entity, &Pleb)>
 ) { 
    for (entity, pleb) in query.iter() {
        if pleb.energy == 0 { 
            commands.entity(entity).despawn();
        }
    }
 }

 fn act(
    mut query: Query<(&mut Transform, &Pleb)>
 ) {
    for (mut transform, pleb) in query.iter_mut() { 
        let action = pleb.network.compute();

        match action { 
            Action::Walk(direction) => {
                transform.translation += direction.to_velocity();
            }
            Action::Stay => { 

            }
        }
    }
 }