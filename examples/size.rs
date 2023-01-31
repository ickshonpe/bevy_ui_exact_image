use bevy::prelude::*;
use bevy_ui_exact_image::*;

fn spawn_example(mut commands: Commands, assets: Res<AssetServer>) {
    let node_size = Size::new(Val::Px(128.0), Val::Px(192.0));
    let alignment = ImageAlignment::TopCenter;
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::FillNode,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    size: node_size,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));
            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::Texture,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    size: node_size,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));

            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::AttemptPreserveAspectRatio,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    size: node_size,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));

            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::ForcePreserveAspectRatio,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    size: node_size,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));

            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: node_size,
                        overflow: Overflow::Hidden,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn((ExactImageBundle {
                        image: ExactImage {
                            texture: assets.load("orientation.png"),
                            size: ExactSize::ForcePreserveAspectRatio,
                            alignment,
                            ..Default::default()
                        },
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            size: node_size,
                            ..Default::default()
                        },
                        ..Default::default()
                    },));
                });
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(ExactImagePlugin)
        .add_startup_system(spawn_example)
        .run();
}
