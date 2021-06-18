// extern

extern crate pathfinding;
extern crate rand;

// mod

mod lib;

// use

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::pass::ClearColor;
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, RayCastMesh, RayCastMethod, RayCastSource, RaycastSystem,
};

use rand::Rng;

use pathfinding::prelude::astar;

use lib::physical::StateOfMatter;
use lib::space::Direction;
use lib::space::Orientation;
use lib::space::I3;
use lib::terrain::Block;
use lib::terrain::Chunk;
use lib::terrain::CHUNK_SIDE_F;
use lib::terrain::CHUNK_VOLUME;

// struct

#[derive(Default)]
pub struct Game {
    chunk: Chunk,
    player: Player,
}

#[derive(Default)]
pub struct Player {
    selection: Vec<Entity>,
}

#[derive(Default)]
pub struct Dwarf {
    speed: f32,
    p_target: Vec3,
    path: Option<(Vec<PathNode>, i32)>,
    stance: Stance,
}

impl Dwarf {
    pub fn path_node(&self) -> PathNode {
        PathNode::new(self.stance, I3::from_vec(self.p_target))
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum Stance {
    Standing,
    Climbing(Orientation),
    Swimming,
    Diving {
        // breath: f32,
    },
    Falling {
        // velocity: Vec3,
    },
}

impl Stance {
    pub fn grab_wall(forward: Direction) -> Stance {
        Stance::Climbing(Orientation::new(forward, up))
    }
}

impl Default for Stance {
    fn default() -> Stance {
        Stance::Standing
    }
}

struct Tree;

struct IronOre;

pub struct TerrainRaycastSet;

// fn

// ECS ~ Entity Component System
// Entity : id to attach components to
// Component : a struct of data
// System : a function that runs each frame (unless told otherwise)

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            vsync: false, // vsync is a source of input lag
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 2 })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<TerrainRaycastSet>::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_system(player_cam_movement.system())
        .add_system(player_terrain_click.system())
        // .add_system(player_test_movement.system())
        .add_system(dwarf_movement.system())
        // You will need to pay attention to what order you add systems! Putting them in the wrong
        // order can result in multiple frames of latency. Ray casting should probably happen after
        // the positions of your meshes have been updated in the UPDATE stage.
        .add_system_to_stage(
            CoreStage::PostUpdate,
            update_raycast_with_cursor
                .system()
                .before(RaycastSystem::BuildRays),
        )
        .add_startup_system(setup.system())
        .run();
}

fn update_raycast_with_cursor(
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    // blocks
    for i in 0..CHUNK_VOLUME {
        let (x, y, z) = Chunk::i_to_xyz(i);

        if y > 1 {
            continue;
        }

        if rand::random() {
            continue;
        }
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        game.chunk.blocks[i] = Block::Dirt;

        // cube
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(x, y, z),
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                ..Default::default()
            })
            .insert(RayCastMesh::<TerrainRaycastSet>::default());
    }
    // trees
    for _ in 0..10 {
        let x = (rng.gen::<f32>() * CHUNK_SIDE_F).floor();
        let z = (rng.gen::<f32>() * CHUNK_SIDE_F).floor();
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
    // ores
    for _ in 0..10 {
        let x = (rng.gen::<f32>() * CHUNK_SIDE_F).floor();
        let z = (rng.gen::<f32>() * CHUNK_SIDE_F).floor();
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.25,
                    subdivisions: 4,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("444444").unwrap(),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(x, 1.0, z),
                ..Default::default()
            })
            .insert(IronOre {});
    }
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 20.0, 20.0).looking_at(
                Vec3::new(0.0, 0.0, 0.0),
                // Vec3::new(CHUNK_SIDE_F / 2.0, 0.0, CHUNK_SIDE_F / 2.0),
                Vec3::Y,
            ),
            ..Default::default()
        })
        .insert(RayCastSource::<TerrainRaycastSet>::new_transform_empty());
    // dwarf
    let dwarf = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.25,
                subdivisions: 4,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("ffd891").unwrap(),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Dwarf {
            speed: 7.0,
            p_target: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .id();
    game.player.selection.push(dwarf);
    // ui
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Test.".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "Häst.".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn dwarf_movement(time: Res<Time>, mut query: Query<(&mut Dwarf, &mut Transform)>) {
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

pub fn click_block_inside(query: &Query<&RayCastSource<TerrainRaycastSet>>) -> Option<I3> {
    if let Ok(raycast_source) = query.single() {
        if let Some(top_intersection) = raycast_source.intersect_top() {
            let p = (top_intersection.1.normal_ray().origin()
                - top_intersection.1.normal_ray().direction() * 0.5)
                .round();
            return Some(I3::from_vec(p));
        }
    }
    None
}

pub fn click_block_outside(query: &Query<&RayCastSource<TerrainRaycastSet>>) -> Option<I3> {
    if let Ok(raycast_source) = query.single() {
        if let Some(top_intersection) = raycast_source.intersect_top() {
            let p = (top_intersection.1.normal_ray().origin()
                + top_intersection.1.normal_ray().direction() * 0.5)
                .round();
            return Some(I3::from_vec(p));
        }
    }
    None
}

fn player_terrain_click(
    mouse_button_input: Res<Input<MouseButton>>,
    game: Res<Game>,
    mut dwarf_query: Query<&mut Dwarf>,
    raycast_source_query: Query<&RayCastSource<TerrainRaycastSet>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // click_block_inside(&raycast_source_query);
    if let Some(p) = click_block_outside(&raycast_source_query) {
        for e in game.player.selection.iter() {
            if let Ok(mut d) = dwarf_query.get_mut(*e) {
                let start = d.path_node();
                let goal = PathNode::from_pos(p);
                d.path = path_between(&game, start, goal);
                d.p_target = p.as_vec();
            }
        }
    }
}

fn player_cam_movement(
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
                transform.translation = transform.translation + vec;
            }
        }
    }
}

// /// Legend:
// /// @: Dwarf
// /// X: Supporting block
// /// O: Non-supporting block
// /// .: Air
// pub enum DwarfMoveCommand {
//     /// Standing -> Standing
//     /// @.    .@
//     /// XX -> XX
//     Walk(I3),
//     /// Standing -> Climbing
//     /// @O    @X
//     /// X  -> O
//     StartClimb(AbsoluteFacingXZ),
//     /// Climbing -> Standing
//     /// ..    @.    .@
//     /// @X -> .X -> .X
//     EndClimbUp,
//     /// Climbing -> Standing
//     /// @X    @O
//     /// O  -> X
//     EndClimbDown,
//     /// Climbing -> Falling
//     DropFromClimb,
//     /// From above:
//     /// @O    @X
//     /// X  -> O
//     RotateClimb(AbsoluteFacingXZ),
//     /// @..    .@.
//     /// X   -> X
//     JumpForward,
//     /// @.    .@
//     /// X. ->  X
//     JumpDown,
//     /// Accelerated by gravity, otherwise constant speed
//     Fall { velocity: Vec3 },
//     /// @.    .@
//     ///  X ->  X
//     LandFall,
// }

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct PathNode {
    pub stance: Option<Stance>,
    pub p: I3,
}

impl PathNode {
    pub fn new(stance: Stance, p: I3) -> PathNode {
        PathNode {
            stance: Some(stance),
            p,
        }
    }
    pub fn from_pos(p: I3) -> PathNode {
        PathNode { stance: None, p }
    }
}

fn path_between(game: &Game, start: PathNode, goal: PathNode) -> Option<(Vec<PathNode>, i32)> {
    astar(
        &start,
        |node| {
            let mut result = Vec::new();

            let p0 = node.p;
            let stance = node.stance.unwrap();

            // TODO: loop through all "capabilities" (walk, dive, swim, climb, jump, etc etc) in order of preference and attempt them

            match stance {
                Stance::Standing => {
                    for &dir in Direction::all_xz().iter() {
                        let p = p0 + dir.i3();
                        let s_p = game.chunk.at_local(p).map(Block::state_of_matter);
                        let s_ny = game.chunk.at_local(p.ny()).map(Block::state_of_matter);
                        if s_p == None {
                            continue;
                        }
                        let s_p = s_p.unwrap();
                        // Capability "walk"
                        if s_p == StateOfMatter::Gas {
                            if s_ny == Some(StateOfMatter::Solid) {
                                // TODO constructor
                                result.push((PathNode::new(Stance::Standing, p), 1));
                            }
                        // Capability "climb"
                        } else if s_p == StateOfMatter::Solid {
                            result.push((
                                // TODO constructor
                                PathNode::new(Stance::grab_wall(dir), p0),
                                1,
                            ));
                        }
                    }
                }
                Stance::Climbing(orientation) => {
                    let grab_dir = orientation.forward;
                    let p_wall = p0 + grab_dir.i3();
                    // Capability "climb" (mount ledge)
                    // Capability "climb" (drop down)
                    // Capability "climb" (turn in tile)
                    // Capability "climb" (along wall)
                    // for &dir in grab_dir.surrounding_4().iter() {
                    //     let p = p0 + dir.i3();
                    //     let s_p = game.chunk.at_local(p).map(Block::state_of_matter);
                    //     let s_ny = game.chunk.at_local(p.ny()).map(Block::state_of_matter);
                    // }
                }
                _ => panic!("TODO stance"),
            }

            result
        },
        |node| {
            let p = node.p;
            p.x * p.x + p.y * p.y + p.z * p.z
        },
        |node| {
            let p = node.p;
            p == goal.p
        },
    )
}

// fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
//     let mut text = query.single_mut().unwrap();
//     text.sections[0].value = format!("Score: {}", scoreboard.score);
// }

// fn ball_collision_system(
//     mut commands: Commands,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
//     collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
// ) {
//     if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
//         let ball_size = sprite.size;
//         let velocity = &mut ball.velocity;

//         // check collision with walls
//         for (collider_entity, collider, transform, sprite) in collider_query.iter() {
//             let collision = collide(
//                 ball_transform.translation,
//                 ball_size,
//                 transform.translation,
//                 sprite.size,
//             );
//             if let Some(collision) = collision {
//                 // scorable colliders should be despawned and increment the scoreboard on collision
//                 if let Collider::Scorable = *collider {
//                     scoreboard.score += 1;
//                     commands.entity(collider_entity).despawn();
//                 }

//                 // reflect the ball when it collides
//                 let mut reflect_x = false;
//                 let mut reflect_y = false;

//                 // only reflect if the ball's velocity is going in the opposite direction of the
//                 // collision
//                 match collision {
//                     Collision::Left => reflect_x = velocity.x > 0.0,
//                     Collision::Right => reflect_x = velocity.x < 0.0,
//                     Collision::Top => reflect_y = velocity.y < 0.0,
//                     Collision::Bottom => reflect_y = velocity.y > 0.0,
//                 }

//                 // reflect velocity on the x-axis if we hit something on the x-axis
//                 if reflect_x {
//                     velocity.x = -velocity.x;
//                 }

//                 // reflect velocity on the y-axis if we hit something on the y-axis
//                 if reflect_y {
//                     velocity.y = -velocity.y;
//                 }

//                 // break if this collide is on a solid, otherwise continue check whether a solid is
//                 // also in collision
//                 if let Collider::Solid = *collider {
//                     break;
//                 }
//             }
//         }
//     }
// }
// use bevy::prelude::*;

// struct Person;

// struct Name(String);

// pub struct HelloPlugin;

// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//             .add_startup_system(add_people.system())
//             .add_system(hello_world.system())
//             .add_system(greet_people.system());
//     }
// }

// fn hello_world() {
//     println!("hello world!");
// }

// fn add_people(mut commands: Commands) {
//     commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
//     commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
//     commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
// }

// struct GreetTimer(Timer);

// fn greet_people(
//     time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     // update our timer with the time elapsed since the last update
//     // if that caused the timer to finish, we say hello to everyone
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// fn main() {
//     App::build()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(HelloPlugin)
//         .run();
// }
