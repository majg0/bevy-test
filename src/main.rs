// extern

extern crate pathfinding;
extern crate rand;

// mod

mod lib;

// use

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastSource};

use lib::debug::DebugPlugin;
use lib::player::PlayerPlugin;
use lib::space::I3;
use lib::tasking::TaskingPlugin;
use lib::terrain::Map;
use lib::terrain::TerrainPlugin;
use lib::terrain::TerrainRaycastSet;
use lib::unit::UnitPlugin;

// fn

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            vsync: false, // vsync is a source of input lag
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 2 })
        .insert_resource(Map::new(I3::new(30, 10, 10)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<TerrainRaycastSet>::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugin(PlayerPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(TaskingPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
