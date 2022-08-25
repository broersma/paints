use bevy::prelude::{App, DefaultPlugins};

mod paints;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(paints::PaintsPlugin)
        .run();
}
