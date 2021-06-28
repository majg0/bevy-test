use bevy::prelude::AppBuilder;
use bevy::prelude::IntoSystem;
use bevy::prelude::Plugin;

use crate::lib::tasking::system::schedule_tasks;

pub struct TaskingPlugin;

impl Plugin for TaskingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(schedule_tasks.system());
    }
}
