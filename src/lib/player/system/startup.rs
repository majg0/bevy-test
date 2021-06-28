use bevy::prelude::*;

use crate::lib::player::LocalPlayer;
use crate::lib::player::PlayerState;
use crate::lib::tasking::TaskProcessor;
use crate::lib::tasking::TaskScheduler;
use crate::lib::unit::Dwarf;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let e_player = commands
        .spawn()
        .insert(PlayerState::default())
        .insert(LocalPlayer::new("Player One".to_string()))
        .insert(TaskScheduler::default())
        .id();
    let e_dwarf = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.25,
                subdivisions: 4,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("ffd891").unwrap(),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Dwarf {
            speed: 7.0,
            p_target: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(TaskProcessor::default())
        .id();
    commands.entity(e_player).push_children(&[e_dwarf]);
}
