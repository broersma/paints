use bevy::prelude::*;

pub struct PaintsPlugin;

mod in_game;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    // TODO: ScoreScreen,
}

impl Plugin for PaintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::MainMenu)
            .add_plugin(menu::PaintsPlugin)
            .add_plugin(in_game::PaintsPlugin)
            .add_startup_system(setup);
    }
}

#[derive(Deref)]
pub struct BucketTexture(Handle<Image>);
#[derive(Deref)]
pub struct PaintTexture(Handle<Image>);
#[derive(Deref)]
pub struct LabelTexture(Handle<Image>);
#[derive(Deref)]
pub struct IconTexture(Handle<Image>);
#[derive(Deref)]
pub struct NozzleTexture(Handle<Image>);

#[derive(Deref)]
pub struct GameFont(Handle<Font>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.insert_resource(IconTexture(asset_server.load("icon.png")));
    commands.insert_resource(BucketTexture(asset_server.load("bucket.png")));
    commands.insert_resource(PaintTexture(asset_server.load("paint.png")));
    commands.insert_resource(LabelTexture(asset_server.load("label.png")));
    commands.insert_resource(NozzleTexture(asset_server.load("nozzle.png")));
    commands.insert_resource(GameFont(asset_server.load("fonts/savate-regular.otf")));
}
