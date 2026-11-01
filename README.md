# Water Off a Duck's Back
## Description:
Allows for easy error handling in bevy.
## Warning: Darkness
This crate requires nightly, as it uses [try_trait_v2](https://github.com/rust-lang/rust/issues/84277) to convert from results and options to `()` using the `?` operator.\
It also uses [try_as_dyn](https://github.com/rust-lang/rust/issues/144361) in order to prefer `Display` to `Debug` implementations when displaying an error.\
I have little doubt that eventually in some form these features will be stabilised.
## Example:
```rust
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
```
```
2026-02-16T05:05:12.958909Z ERROR duck_back: (examples/messages.rs:20:5)
Failed to unwrap value.
No entities fit the query bevy_ecs::system::query::Query<'_, '_, &bevy_transform::components::transform::Transform>
2026-02-16T05:05:12.958934Z TRACE duck_back: (examples/messages.rs:16:5)
Failed to unwrap value.
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

