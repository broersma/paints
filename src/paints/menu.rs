use bevy::app::AppExit;
use bevy::prelude::*;

use super::{AppState, GameFont, IconTexture};

pub struct PaintsPlugin;

impl Plugin for PaintsPlugin {
    fn build(&self, app: &mut App) {
        app
            // systems to run only in the main menu
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(on_update))
            // setup when entering the state
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(on_enter))
            // cleanup when exiting the state
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(on_exit));
    }
}

#[derive(Component)]
pub struct MenuTitle;

fn on_enter(mut commands: Commands, game_font: Res<GameFont>, icon_texture: Res<IconTexture>) {
    commands
        .spawn_bundle(TextBundle::from_section(
            "[Space] to play,\n[Esc] to exit",
            TextStyle {
                font: game_font.clone_weak(),
                font_size: 64.0,
                color: Color::WHITE,
            },
        ))
        .insert(MenuTitle);

    commands.spawn_bundle(SpriteBundle {
        texture: icon_texture.clone_weak(),
        ..Default::default()
    })
    .insert(MenuTitle);
}

fn on_update(
    mut app_state: ResMut<State<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
        keyboard_input.reset(KeyCode::Space);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MenuTitle>>) {
    for text_entity in query.iter() {
        commands.entity(text_entity).despawn()
    }
}
