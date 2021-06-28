use bevy::prelude::Query;
use bevy::prelude::Vec3;
use bevy_mod_raycast::Intersection;
use bevy_mod_raycast::RayCastSource;

use crate::lib::space::I3;

// TODO: raycasting API structure needs some thought...

pub struct TerrainRaycastSet;

pub struct TerrainRayHit {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl TerrainRayHit {
    pub fn new(intersection: Intersection) -> TerrainRayHit {
        TerrainRayHit {
            origin: intersection.normal_ray().origin(),
            direction: intersection.normal_ray().direction(),
        }
    }
    pub fn inside(&self) -> I3 {
        let p = (self.origin - self.direction * 0.5).round();
        I3::from_vec(p)
    }
    pub fn outside(&self) -> I3 {
        let p = (self.origin + self.direction * 0.5).round();
        I3::from_vec(p)
    }
}

pub fn raycast_terrain(query: &Query<&RayCastSource<TerrainRaycastSet>>) -> Option<TerrainRayHit> {
    if let Ok(raycast_source) = query.single() {
        if let Some(top_intersection) = raycast_source.intersect_top() {
            return Some(TerrainRayHit::new(top_intersection.1));
        }
    }
    None
}
