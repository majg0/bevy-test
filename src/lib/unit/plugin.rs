use bevy::prelude::AppBuilder;
use bevy::prelude::IntoSystem;
use bevy::prelude::Plugin;

use crate::lib::unit::system::movement;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement.system());
    }
}
