use bevy::prelude::*;

use crate::lib::player::Player;
use crate::lib::player::PlayerState;
use crate::lib::player::Players;
use crate::lib::player::Selection;
use crate::lib::tasking::TaskSet;
use crate::lib::unit::Dwarf;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dwarf = commands
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
        .id();
    let selection = Selection(vec![dwarf]);
    let local_player_one = commands
        .spawn()
        .insert(selection)
        .insert(PlayerState::default())
        .insert(Player::new("Player One".to_string()))
        .insert(TaskSet::default())
        .id();

    commands.insert_resource(Players {
        all_players: vec![local_player_one],
        local_players: vec![local_player_one],
    });
}
