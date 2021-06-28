use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use bevy::prelude::Vec3;
use bevy::render::camera::Camera;

pub fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (key, vec) in [
        (KeyCode::Left, Vec3::new(-1.0, 0.0, 0.0)),
        (KeyCode::Right, Vec3::new(1.0, 0.0, 0.0)),
        (KeyCode::Up, Vec3::new(0.0, 0.0, -1.0)),
        (KeyCode::Down, Vec3::new(0.0, 0.0, 1.0)),
    ] {
        if keyboard_input.pressed(key) {
            for (mut transform, _) in query.iter_mut() {
                transform.translation += vec;
            }
        }
    }
}
