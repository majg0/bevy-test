use bevy::app::AppExit;
use bevy::prelude::EventWriter;
use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::Res;

pub fn quit(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}
