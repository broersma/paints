use bevy::prelude::*;
use bevy::time::FixedTimestep;

use rand::random;

use super::{AppState, BucketTexture, GameFont, LabelTexture, NozzleTexture, PaintTexture};

const TIME_STEP: f32 = 1.0 / 60.0;
const PAINT_BUCKET_IMAGE_WIDTH: f32 = 315.;
const PAINT_BUCKET_WIDTH: f32 = 168.;
// TODO: const PAINT_BUCKET_SPEED: f32 = 150.;
const PAINT_BUCKET_SPEED: f32 = 300.;
const PAINT_BUCKET_SPAWN_DELAY: f32 = PAINT_BUCKET_WIDTH / PAINT_BUCKET_SPEED * 2.;

// TODO: const PAINT_BUCKET_SPAWN_MAX: u32 = 50;
const PAINT_BUCKET_SPAWN_MAX: u32 = 5;

pub struct PaintsPlugin;

impl Plugin for PaintsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .insert_resource(GameState {
                time_since_last_paint_bucket_spawn: PAINT_BUCKET_SPAWN_DELAY,
                ..Default::default()
            })
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(on_update))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(on_fixed_update),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(paint_sprite_coloring_system),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(paint_label_sprite_coloring_system),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(paint_bucket_spawner),
            )
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(paint_bucket_scorer))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(on_exit));
    }
}

#[derive(Component)]
struct Moving {
    /// linear speed in meters per second
    movement_speed: f32,
}

#[derive(Component)]
struct PaintLabel {
    color: Color,
}
#[derive(Component)]
struct Paint {
    color: Color,
}

#[derive(Component)]
struct PaintBucket;

#[derive(Component)]
pub struct PausedTitle;
#[derive(Component)]
pub struct ScoreTitle;

#[derive(Component)]
struct PaintNozzle;

#[derive(Default)]
pub struct GameState {
    is_paused: bool,
    time_since_last_paint_bucket_spawn: f32,
    buckets_spawned: u32,
    buckets_scored: u32,
    score: f32,
    show_score: bool,
}

fn on_enter(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    game_font: Res<GameFont>,
    nozzle_texture: Res<NozzleTexture>,
    label_texture: Res<LabelTexture>,
) {
    game_state.is_paused = false;
    game_state.time_since_last_paint_bucket_spawn = PAINT_BUCKET_SPAWN_DELAY;
    game_state.buckets_spawned = 0;
    game_state.buckets_scored = 0;
    game_state.score = 0.;
    game_state.show_score = false;

    commands
        .spawn_bundle(TextBundle::from_section(
            "Paused\n\n[Escape] to continue\n[Return] to go to main menu",
            TextStyle {
                font: game_font.clone_weak(),
                font_size: 64.0,
                color: Color::WHITE,
            },
        ))
        .insert(Visibility { is_visible: false })
        .insert(PausedTitle);

    commands
        .spawn_bundle(TextBundle::from_section(
            "Score: 000\n[Return] to go to main menu",
            TextStyle {
                font: game_font.clone_weak(),
                font_size: 64.0,
                color: Color::WHITE,
            },
        ))
        .insert(Visibility { is_visible: false })
        .insert(ScoreTitle);

    spawn_paint_nozzle(
        &mut commands,
        nozzle_texture.clone_weak(),
        label_texture.clone_weak(),
        Color::Rgba {
            red: 1.,
            green: 0.,
            blue: 0.,
            alpha: 1.,
        },
        -200.,
    );

    spawn_paint_nozzle(
        &mut commands,
        nozzle_texture.clone_weak(),
        label_texture.clone_weak(),
        Color::Rgba {
            red: 0.,
            green: 1.,
            blue: 0.,
            alpha: 1.,
        },
        0.,
    );

    spawn_paint_nozzle(
        &mut commands,
        nozzle_texture.clone_weak(),
        label_texture.clone_weak(),
        Color::Rgba {
            red: 0.,
            green: 0.,
            blue: 1.,
            alpha: 1.,
        },
        200.,
    );
}

fn on_update(
    mut app_state: ResMut<State<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    windows: Res<Windows>,
    bucket_texture: Res<BucketTexture>,
    paint_texture: Res<PaintTexture>,
    label_texture: Res<LabelTexture>,
    mut query: Query<&mut Visibility, (With<PausedTitle>, Without<ScoreTitle>)>,
    mut score_query: Query<(&mut Visibility, &mut Text), (With<ScoreTitle>, Without<PausedTitle>)>,
) {
    if game_state.show_score {
        let (mut score_visibility, mut text) = score_query.single_mut();
        score_visibility.is_visible = true;
        let final_score = game_state.score * 1000.;
        text.sections[0].value = format!("Score: {final_score:06.0}\n[Return] to go to main menu");
        if keyboard_input.just_pressed(KeyCode::Return) {
            app_state.set(AppState::MainMenu).unwrap();
            keyboard_input.reset(KeyCode::Return);
        }
    } else {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            game_state.is_paused = !game_state.is_paused;
            query.single_mut().is_visible = game_state.is_paused;
            keyboard_input.reset(KeyCode::Return);
        } else if game_state.is_paused && keyboard_input.just_pressed(KeyCode::Return) {
            app_state.set(AppState::MainMenu).unwrap();
            keyboard_input.reset(KeyCode::Return);
        }
    }
}

fn on_exit(
    mut commands: Commands,
    query: Query<Entity, Or<(With<PaintBucket>,With<PaintNozzle>, With<PausedTitle>, With<ScoreTitle>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}

fn on_fixed_update(game_state: Res<GameState>, mut query: Query<(&Moving, &mut Transform)>) {
    if !game_state.is_paused {
        query.for_each_mut(|(paint_bucket, mut transform)| {
            let movement_direction = Vec3::X;
            let movement_distance = paint_bucket.movement_speed * TIME_STEP;
            let translation_delta = movement_direction * movement_distance;
            transform.translation += translation_delta;
        });
    }
}

fn paint_bucket_spawner(
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    time: Res<Time>,
    windows: Res<Windows>,
    bucket_texture: Res<BucketTexture>,
    paint_texture: Res<PaintTexture>,
    label_texture: Res<LabelTexture>,
) {
    if !game_state.is_paused {
        game_state.time_since_last_paint_bucket_spawn += time.delta_seconds()
    }

    if game_state.buckets_spawned < PAINT_BUCKET_SPAWN_MAX
        && game_state.time_since_last_paint_bucket_spawn > PAINT_BUCKET_SPAWN_DELAY
    {
        spawn_paint_bucket(
            &mut commands,
            &windows,
            &time,
            bucket_texture.clone_weak(),
            paint_texture.clone_weak(),
            label_texture.clone_weak(),
        );
        game_state.time_since_last_paint_bucket_spawn -= PAINT_BUCKET_SPAWN_DELAY;
        game_state.buckets_spawned += 1;
    }
}

fn paint_bucket_scorer(
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    windows: Res<Windows>,
    paint_bucket_query: Query<(Entity, &Transform, &Children), With<PaintBucket>>,
    paint_query: Query<&Paint>,
    paint_label_query: Query<&PaintLabel>,
) {
    let window = windows.primary();
    // get the properties of each squad
    for (entity, transform, children) in paint_bucket_query.iter() {
        if paint_bucket_went_off_screen(transform, window) {
            let paint_and_label = get_paint_and_paint_label(entity, children, &paint_query, &paint_label_query);

            if let Some((paint, paint_label)) = paint_and_label {
                game_state.score += calculate_score(paint, paint_label);
                game_state.buckets_scored += 1;

                // despawn the paint bucket
                commands.entity(entity).despawn_recursive();

                // did we score all paint buckets?
                if game_state.buckets_scored == PAINT_BUCKET_SPAWN_MAX {
                    game_state.show_score = true;
                }
            }
        }
    }
}

fn get_paint_and_paint_label<'a>(entity: Entity, children: &'a Children, paint_query: &'a Query<&Paint>, paint_label_query: &'a Query<&PaintLabel>) -> Option<(&'a Paint, &'a PaintLabel)> {
    
    // TODO: this is bad Rust...

    let mut paint_result = Err(bevy::ecs::query::QueryEntityError::NoSuchEntity(entity));
    let mut paint_label_result =
        Err(bevy::ecs::query::QueryEntityError::NoSuchEntity(entity));
    for &child in children.iter() {
        if paint_query.get(child).is_ok() {
            paint_result = paint_query.get(child);
        }

        if paint_label_query.get(child).is_ok() {
            paint_label_result = paint_label_query.get(child);
        }
    }
    
    if let (Ok(paint), Ok(paint_label)) = (paint_result, paint_label_result) {
        Some((paint, paint_label))
    } else {
        None
    }
}

fn calculate_score(paint: &Paint, paint_label: &PaintLabel) -> f32 {
    ((paint.color.r() - paint_label.color.r()).powf(2.)
        + (paint.color.g() - paint_label.color.g()).powf(2.)
        + (paint.color.b() - paint_label.color.b()).powf(2.))
    .sqrt()
        / (3f32).sqrt()
}

fn paint_bucket_went_off_screen(transform: &Transform, window: &Window) -> bool {
    transform.translation.x > (window.width() as f32 / 2.) + PAINT_BUCKET_WIDTH * 0.5
}

fn paint_sprite_coloring_system(mut query: Query<(&mut Sprite, &Paint)>) {
    for (mut sprite, paint) in query.iter_mut() {
        sprite.color = paint.color;
    }
}

fn paint_label_sprite_coloring_system(mut query: Query<(&mut Sprite, &PaintLabel)>) {
    for (mut sprite, paint) in query.iter_mut() {
        sprite.color = paint.color;
    }
}

fn spawn_paint_bucket(
    commands: &mut Commands,
    windows: &Windows,
    time: &Time,
    bucket_texture: Handle<Image>,
    paint_texture: Handle<Image>,
    label_texture: Handle<Image>,
) {
    let window = windows.primary();
    let bucket_x = (window.width() as f32 / -2.) - PAINT_BUCKET_WIDTH * 0.5;
    let bucket_y = 0.;
    let seconds = time.seconds_since_startup() as f32;

    commands
        .spawn_bundle(SpriteBundle {
            texture: bucket_texture,
            transform: Transform::from_xyz(bucket_x, bucket_y, 0.),
            ..Default::default()
        })
        .insert(PaintBucket)
        .insert(Moving {
            movement_speed: PAINT_BUCKET_SPEED, // metres per second
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: paint_texture,
                    ..Default::default()
                })
                .insert(Paint {
                    color: Color::Rgba {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                        alpha: 1.0,
                    },
                });

            parent
                .spawn_bundle(SpriteBundle {
                    texture: label_texture,
                    ..Default::default()
                })
                .insert(PaintLabel {
                    color: Color::Rgba {
                        red: (1.25 * seconds + 10. * random::<f32>()).sin() / 2.0 + 0.5,
                        green: (0.75 * seconds + 10. * random::<f32>()).sin() / 2.0 + 0.5,
                        blue: (0.50 * seconds + 10. * random::<f32>()).sin() / 2.0 + 0.5,
                        alpha: 1.0,
                    },
                });
        });
}

fn spawn_paint_nozzle(
    commands: &mut Commands,
    nozzle_texture: Handle<Image>,
    label_texture: Handle<Image>,
    color: Color,
    position_x: f32,
) {
    let nozzle_x = position_x;
    let nozzle_y = 200.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: nozzle_texture,
            transform: Transform::from_xyz(nozzle_x, nozzle_y, 0.),
            ..Default::default()
        })
        .insert(PaintNozzle)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: label_texture,
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..Default::default()
                })
                .insert(PaintLabel { color: color });
        });
}
