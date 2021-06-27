use bevy::{input::system::exit_on_esc_system, prelude::*};

mod notes;
mod consts;
mod types;
mod ui;

use notes::NotesPlugin;
use ui::UIPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Taikrust".to_string(),
            width: 1366.,
            height: 768.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(NotesPlugin)
        .add_plugin(UIPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    let config = types::load_config();

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .insert_resource(config);
}