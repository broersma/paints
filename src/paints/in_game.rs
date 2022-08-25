use bevy::prelude::*;
use bevy::time::FixedTimestep;

use super::{AppState, BucketTexture, LabelTexture, PaintTexture};

const TIME_STEP: f32 = 1.0 / 60.0;
const PAINT_BUCKET_WIDTH: f32 = 315.;

pub struct PaintsPlugin;

impl Plugin for PaintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(paint_bucket_coloring_system)
            .add_system(color_changing_system)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(on_update))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(on_fixed_update),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(on_exit));
    }
}

#[derive(Component)]
struct Moving {
    /// linear speed in meters per second
    movement_speed: f32,
}

#[derive(Component)]
struct Paint {
    color: Color,
}

fn on_enter() {
    // TODO: spawn paint nozzle
}

fn on_update(
    mut app_state: ResMut<State<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut commands: Commands,
    windows: Res<Windows>,
    bucket_texture: Res<BucketTexture>,
    paint_texture: Res<PaintTexture>,
    label_texture: Res<LabelTexture>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        keyboard_input.reset(KeyCode::Escape);
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        spawn_paint_bucket(
            &mut commands,
            &windows,
            bucket_texture.clone_weak(),
            paint_texture.clone_weak(),
            label_texture.clone_weak(),
        );
    }
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<Paint>>) {
    for paint_entity in query.iter() {
        commands.entity(paint_entity).despawn();
    }
}

fn on_fixed_update(mut query: Query<(&Moving, &mut Transform)>) {
    query.for_each_mut(|(paint_bucket, mut transform)| {
        let movement_direction = Vec3::X;
        let movement_distance = paint_bucket.movement_speed * TIME_STEP;
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
    });
}

fn paint_bucket_coloring_system(mut query: Query<(&mut Sprite, &Paint)>) {
    for (mut sprite, paint) in query.iter_mut() {
        sprite.color = paint.color;
    }
}

fn color_changing_system(time: Res<Time>, mut query: Query<(&mut Paint, &Name)>) {
    let mut seconds = time.seconds_since_startup() as f32;

    for (mut paint, name) in query.iter_mut() {
        let sum_of_bytes: f32 = name.bytes().fold(0u8, |a, b| a.wrapping_add(b)).into();
        seconds += sum_of_bytes;
        paint.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn spawn_paint_bucket(
    commands: &mut Commands,
    windows: &Windows,
    bucket_texture: Handle<Image>,
    paint_texture: Handle<Image>,
    label_texture: Handle<Image>,
) {
    let window = windows.primary();
    let bird_x = (window.width() as f32 / -2.) - PAINT_BUCKET_WIDTH * 0.5;
    let bird_y = 0.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: bucket_texture,
            transform: Transform::from_xyz(bird_x, bird_y, 0.),
            ..Default::default()
        })
        .insert(Moving {
            movement_speed: 100.0, // metres per second
        })
        .insert(Paint {
            color: Color::Rgba {
                red: 0.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
        })
        .insert(Name::new("Paint bucket"))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: paint_texture,
                    ..Default::default()
                })
                .insert(Paint {
                    color: Color::Rgba {
                        red: 1.0,
                        green: 0.0,
                        blue: 1.0,
                        alpha: 1.0,
                    },
                })
                .insert(Name::new("Paint"));

            parent
                .spawn_bundle(SpriteBundle {
                    texture: label_texture,
                    ..Default::default()
                })
                .insert(Paint {
                    color: Color::Rgba {
                        red: 1.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 1.0,
                    },
                })
                .insert(Name::new("Label"));
        });
}
