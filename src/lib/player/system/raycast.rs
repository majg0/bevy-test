use bevy::prelude::CursorMoved;
use bevy::prelude::EventReader;
use bevy::prelude::Input;
use bevy::prelude::MouseButton;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy_mod_raycast::RayCastMethod;
use bevy_mod_raycast::RayCastSource;

// use crate::lib::pathfinding::PathNode;
use crate::lib::player::LocalPlayer;
use crate::lib::tasking::TaskScheduler;
use crate::lib::terrain::raycast_terrain;
use crate::lib::terrain::TerrainRaycastSet;
// use crate::lib::terrain::Chunk;
// use crate::lib::terrain::Map;
// use crate::lib::unit::Dwarf;

pub fn raycast(
    mut task_scheduler_query: Query<(&LocalPlayer, &mut TaskScheduler)>,
    mouse_button_input: Res<Input<MouseButton>>,
    raycast_source_query: Query<&RayCastSource<TerrainRaycastSet>>,
    // map: Res<Map>,
    // mut dwarf_query: Query<&mut Dwarf>,
    // chunk_query: Query<&Chunk>,
) {
    for (_, mut task_scheduler) in task_scheduler_query.iter_mut() {
        // TODO: key bindings?
        if !mouse_button_input.just_pressed(MouseButton::Left) {
            return;
        }

        // click_block_inside(&raycast_source_query);
        if let Some(hit) = raycast_terrain(&raycast_source_query) {
            let p = hit.outside();
            task_scheduler.remove_block(p);
            dbg!(task_scheduler);
            // for e in selection.0.iter() {
            //     if let Ok(mut d) = dwarf_query.get_mut(*e) {
            //         let start = d.path_node();
            //         let goal = PathNode { stance: None, p };
            //         d.path = start.path_to(&map, &chunk_query, goal);
            //         d.p_target = p.vec3();
            //     }
            // }
        }
    }
}

// TODO: move?

pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<TerrainRaycastSet>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}
