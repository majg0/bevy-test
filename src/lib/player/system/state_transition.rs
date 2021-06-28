use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::Query;
use bevy::prelude::Res;

use crate::lib::player::LocalPlayer;
use crate::lib::player::PlayerState;

pub fn state_transition(
    mut player_state_query: Query<(&LocalPlayer, &mut PlayerState)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (_, mut player_state) in player_state_query.iter_mut() {
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
