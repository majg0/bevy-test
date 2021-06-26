use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;

use rand::Rng;

use crate::lib::structure::Tree;
use crate::lib::terrain::Block;
use crate::lib::terrain::Chunk;
use crate::lib::terrain::Map;
use crate::lib::terrain::TerrainRaycastSet;
use crate::lib::terrain::CHUNK_SIDE_POW;
use crate::lib::terrain::CHUNK_VOLUME;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map: ResMut<Map>,
) {
    let mut rng = rand::thread_rng();
    // blocks
    let bounds = map.size_in_blocks;
    for ci in 0..map.num_chunks() {
        let mut chunk: Chunk = Default::default();
        let chunk_p = map.i_to_p(ci);
        let block_p = chunk_p.map(|x| x << CHUNK_SIDE_POW).vec3();
        for i in 0..CHUNK_VOLUME {
            let p = Chunk::i_to_p(i);
            if !p.each_less_than(bounds) {
                continue;
            }

            if p.y > 1 {
                continue;
            }

            if rand::random() {
                continue;
            }
            let x = p.x as f32;
            let y = p.y as f32;
            let z = p.z as f32;
            chunk.blocks[i] = Block::Dirt;

            // cube
            commands
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(block_p.x + x, block_p.y + y, block_p.z + z),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    ..Default::default()
                })
                .insert(RayCastMesh::<TerrainRaycastSet>::default());
        }
        let id = commands.spawn().insert(chunk).id();
        map.chunks.push(id);
    }
    // trees
    for _ in 0..10 {
        let x = (rng.gen::<f32>() * bounds.x as f32).floor();
        let z = (rng.gen::<f32>() * bounds.z as f32).floor();
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 4,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("00aa00").unwrap(),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(x, 1.0, z),
                ..Default::default()
            })
            .insert(Tree {});
    }
}
