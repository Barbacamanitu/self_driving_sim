use bevy::{
    prelude::*,
    render::render_resource::{Texture, TextureId},
};

use crate::{car::Car, math::lerp};

const SHOULDER_COLOR: Color = Color::rgba(1.0, 1.0, 0.0, 0.6);
const ASPALT_TEXTURE: &str = "sprites/asphalt.png";
const LANE_DASH_TEXTURE: &str = "sprites/road_lane_dashes.png";
const ROAD_Z: f32 = 10.0;
pub struct RoadPlugin;

#[derive(Debug)]
pub struct RoadConfig {
    pub texture: Handle<Image>,
    pub dashes_texture: Handle<Image>,
    pub inner_width: f32,
    pub left: f32,
    pub right: f32,
    pub lane_count: u32,
}

#[derive(Component)]
pub struct Road;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(road_setup_system);
        app.add_startup_system_to_stage(StartupStage::PostStartup, road_spawn_system);
        app.add_system(road_snap_system);
    }
}

fn road_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asphalt: Handle<Image> = asset_server.load(ASPALT_TEXTURE);
    let dashes: Handle<Image> = asset_server.load(LANE_DASH_TEXTURE);
    let t_size = Vec2::new(1024.0, 4096.0);
    let inner_width = t_size.x * 0.9;
    let left = -inner_width / 2.0;
    let right = inner_width / 2.0;

    let road = RoadConfig {
        texture: asphalt,
        inner_width,
        lane_count: 3,
        left: left,
        right: right,
        dashes_texture: dashes,
    };
    commands.insert_resource(road);
}

fn road_spawn_system(mut commands: Commands, road: Res<RoadConfig>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, ROAD_Z),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(0.5, 0.5, 1.0),
            },
            texture: road.texture.clone(),
            ..default()
        })
        .insert(Road {})
        .with_children(|parent| {
            //Left shoulder
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SHOULDER_COLOR,
                    custom_size: Some(Vec2::new(20.0, 4096.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(road.left, 0.0, 1.0)),
                ..default()
            });

            //right shoulder
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SHOULDER_COLOR,
                    custom_size: Some(Vec2::new(20.0, 4096.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(road.right, 0.0, 1.0)),
                ..default()
            });

            for i in 1..=road.lane_count - 1 {
                let x = lerp(road.left, road.right, i as f32 / road.lane_count as f32);
                parent.spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(x, 0.0, 1.0),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::new(1.0, 1.0, 1.0),
                    },
                    texture: road.dashes_texture.clone(),
                    ..default()
                });
            }
        });
}

fn road_snap_system(
    cars: Query<&Transform, (With<Car>, Without<Road>)>,
    mut roads: Query<&mut Transform, With<Road>>,
) {
    for car_trans in &cars {
        for mut road in &mut roads {
            let car_y = car_trans.translation.y;
            let road_y = (car_y / 1024.0).round() * 1024.0;

            road.translation.y = road_y;
        }
    }
}
