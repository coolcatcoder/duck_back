use bevy::{log::LogPlugin, prelude::*};
use duck_back::Else;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "duck_back=trace".to_string(),
            ..default()
        }))
        .add_systems(Startup, (start, query))
        .run();
}

fn start() {
    let bad: Option<u32> = None;
    bad.else_return()?;
}

fn query(transform: Query<&Transform>) {
    transform.single().else_error()?;
}
