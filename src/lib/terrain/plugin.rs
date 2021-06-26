use bevy::prelude::AppBuilder;
use bevy::prelude::IntoSystem;
use bevy::prelude::Plugin;

use crate::lib::terrain::system::startup;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
