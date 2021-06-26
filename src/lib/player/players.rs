use bevy::prelude::Entity;

#[derive(Default)]
pub struct Players {
    pub all_players: Vec<Entity>,
    pub local_players: Vec<Entity>,
}
