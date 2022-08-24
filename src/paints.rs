use bevy::prelude::*;
use bevy::time::FixedTimestep;

const TIME_STEP: f32 = 1.0 / 60.0;
const PAINT_BUCKET_WIDTH: f32 = 315.;

pub struct PaintsPlugin;

impl Plugin for PaintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(paint_bucket_movement_system),
        )
        .add_system(paint_bucket_coloring_system)
        .add_system(color_changing_system)
        .add_system(mouse_handler)
        .add_startup_system(setup);
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

#[derive(Deref)]
struct BucketTexture(Handle<Image>);
#[derive(Deref)]
struct PaintTexture(Handle<Image>);
#[derive(Deref)]
struct LabelTexture(Handle<Image>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(TextBundle::from_section(
        "Paints (press LMB!)",
        TextStyle {
            font: asset_server.load("fonts/savate-regular.otf"),
            font_size: 64.0,
            color: Color::WHITE,
        },
    ));

    let bucket_texture = BucketTexture(asset_server.load("bucket.png"));
    let paint_texture = PaintTexture(asset_server.load("paint.png"));
    let label_texture = LabelTexture(asset_server.load("label.png"));

    spawn_paint_bucket(
        &mut commands,
        &windows,
        bucket_texture.clone_weak(),
        paint_texture.clone_weak(),
        label_texture.clone_weak(),
    );

    commands.insert_resource(bucket_texture);
    commands.insert_resource(paint_texture);
    commands.insert_resource(label_texture);
}

fn paint_bucket_movement_system(mut query: Query<(&Moving, &mut Transform)>) {
    query.for_each_mut(|(paint_bucket, mut transform)| {
        let movement_direction = Vec3::X;
        let movement_distance = paint_bucket.movement_speed * TIME_STEP;
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
    });
}

fn paint_bucket_coloring_system(mut query: Query<(&mut Sprite, &Paint)>) {
    for (mut sprite, paint) in query.iter_mut() {
        sprite.color = paint.color
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

fn mouse_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    bucket_texture: Res<BucketTexture>,
    paint_texture: Res<PaintTexture>,
    label_texture: Res<LabelTexture>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        spawn_paint_bucket(
            &mut commands,
            &windows,
            bucket_texture.clone_weak(),
            paint_texture.clone_weak(),
            label_texture.clone_weak(),
        );
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
