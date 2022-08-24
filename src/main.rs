use bevy::prelude::{DefaultPlugins, App};

mod paints;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(paints::PaintsPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}