use bevy::prelude::*;



fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Crypt".into(),
                        resolution: (512.0, 512.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build()
        )
        .add_systems(Startup, setup_camera)
        .run()
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
