use bevy::prelude::AppBuilder;
use bevy::prelude::CoreStage;
use bevy::prelude::IntoSystem;
use bevy::prelude::ParallelSystemDescriptorCoercion;
use bevy::prelude::Plugin;
use bevy_mod_raycast::RaycastSystem;

use crate::lib::player::system::camera_movement;
use crate::lib::player::system::raycast;
use crate::lib::player::system::startup;
use crate::lib::player::system::state_transition;
use crate::lib::player::system::update_raycast_with_cursor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(camera_movement.system())
            .add_system(raycast.system())
            .add_system(state_transition.system())
            // You will need to pay attention to what order you add systems! Putting them in the wrong
            // order can result in multiple frames of latency. Ray casting should probably happen after
            // the positions of your meshes have been updated in the UPDATE stage.
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_raycast_with_cursor
                    .system()
                    .before(RaycastSystem::BuildRays),
            )
            .add_startup_system(startup.system());
    }
}
