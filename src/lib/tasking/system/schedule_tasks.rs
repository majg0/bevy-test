use bevy::prelude::Children;
use bevy::prelude::Query;

use crate::lib::tasking::model::TaskProcessor;
use crate::lib::tasking::model::TaskScheduler;
// use crate::lib::terrain::Map;
// use crate::lib::terrain::Chunk;

pub fn schedule_tasks(
    mut q_scheduler: Query<(&mut TaskScheduler, &Children)>,
    mut q_processor: Query<&mut TaskProcessor>,
    // map: Res<Map>,
    // chunk_query: Query<&Chunk>,
) {
    for (mut scheduler, children) in q_scheduler.iter_mut() {
        // TODO: let processor select task by suitability
        for e_processor in children.iter() {
            if let Ok(mut processor) = q_processor.get_mut(*e_processor) {
                if !processor.queue.is_empty() {
                    continue;
                }

                if let Some(task) = scheduler.tasks.pop() {
                    processor.queue.push(task);
                    dbg!(processor);
                }
            }
        }
    }

    // TODO: work stealing

    //     let start = d.path_node();
    //     let goal = PathNode { stance: None, p };
    //     d.path = start.path_to(&map, &chunk_query, goal);
    //     d.p_target = p.vec3();
    // }
    // // click_block_inside(&raycast_source_query);
    // if let Some(hit) = raycast_terrain(&raycast_source_query) {
    //     let p = hit.outside();
    //     task_scheduler.remove_block(p);
    //     dbg!(task_scheduler);
    //     // for e in selection.0.iter() {
    //     //     if let Ok(mut d) = dwarf_query.get_mut(*e) {
    //     //         let start = d.path_node();
    //     //         let goal = PathNode { stance: None, p };
    //     //         d.path = start.path_to(&map, &chunk_query, goal);
    //     //         d.p_target = p.vec3();
    //     //     }
    //     // }
}
