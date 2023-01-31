use bevy::prelude::*;
use bevy_ui_exact_image::*;

fn spawn_example(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((ExactImageBundle {
        image: ExactImage {
            texture: assets.load("orientation.png"),
            color: Color::WHITE,
            ..Default::default()
        },
        style: Style {
            size: Size::new(Val::Px(25.0), Val::Px(25.0)),
            ..Default::default()
        },
        ..Default::default()
    },));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(UiImagePlusPlugin)
        .add_startup_system(spawn_example)
        .run();
}
