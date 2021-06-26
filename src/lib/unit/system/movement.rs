use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::Vec3;

use crate::lib::unit::Dwarf;

pub fn movement(time: Res<Time>, mut query: Query<(&mut Dwarf, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (mut dwarf, mut transform) in query.iter_mut() {
        if dwarf.path.is_none() {
            continue;
        }
        let speed = dwarf.speed;
        let path = dwarf.path.as_mut().unwrap();
        let node = path.0[0];
        // let stance = node.stance;
        let p = node.p;
        let v = Vec3::new(p.x as f32, p.y as f32, p.z as f32);
        let dp = v - transform.translation;
        let max_move = dt * speed;
        if dp.length_squared() >= max_move * max_move {
            transform.translation += max_move * dp.normalize();
        } else {
            if path.0.len() == 1 {
                dwarf.path = None;
            } else {
                path.0.remove(0);
            }
            transform.translation = v;
        }
    }
}
