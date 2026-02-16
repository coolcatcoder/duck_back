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
    let _bad: u32 = bad.else_return()?;
}

fn query(transform: Query<&Transform>) {
    let _transform: &Transform = transform.single().else_error()?;
}
