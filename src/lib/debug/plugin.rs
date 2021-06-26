use bevy::prelude::AppBuilder;
use bevy::prelude::IntoSystem;
use bevy::prelude::Plugin;

use crate::lib::debug::system::quit;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(quit.system());
    }
}
