use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::Query;
use bevy::prelude::Res;

use crate::lib::player::PlayerState;
use crate::lib::player::Players;

pub fn state_transition(
    players: Res<Players>,
    mut player_state_query: Query<&mut PlayerState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for player in players.local_players.iter() {
        if let Ok(mut player_state) = player_state_query.get_mut(*player) {
            // TODO key bindings resource
            for (from, to) in [
                (
                    PlayerState::Select,
                    vec![(KeyCode::R, PlayerState::RemoveBlock)],
                ),
                (
                    PlayerState::RemoveBlock,
                    vec![(KeyCode::Escape, PlayerState::Select)],
                ),
            ]
            .iter()
            {
                if *from == *player_state {
                    for &(key, state) in to.iter() {
                        if keyboard_input.just_pressed(key) {
                            *player_state = state;
                            dbg!(state);
                            return;
                        }
                    }
                    return;
                }
            }
        }
    }
}
