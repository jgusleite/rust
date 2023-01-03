use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
// use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "Rolling Game".into(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins).run();
}
