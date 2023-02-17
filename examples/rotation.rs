use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ui_exact_image::*;

fn spawn_example(mut commands: Commands, assets: Res<AssetServer>) {
    let size = 128.;
    let margin = 0.125 * size;
    let width = 4. * (size + 2. * margin);
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::WHITE.into(),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(width), Val::Percent(100.)),
                        flex_wrap: FlexWrap::Wrap,
                        ..Default::default()
                    },
                    background_color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|builder| {
                    for rotation in (0..16).map(|i| i as f32 * PI / 8.0) {
                        builder.spawn((ExactImageBundle {
                            image: ExactImage {
                                texture: assets.load("orientation.png"),
                                color: Color::WHITE,
                                rotation: Some(rotation),
                                size: ExactSize::Texture,
                                ..Default::default()
                            },
                            style: Style {
                                size: Size::new(Val::Px(size), Val::Px(size)),
                                margin: UiRect::all(Val::Px(margin)),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::PURPLE),
                            ..Default::default()
                        },));
                    }
                });
        });
}

fn rotate_images(time: Res<Time>, mut exact_image_query: Query<&mut ExactImage>) {
    exact_image_query.iter_mut().for_each(|mut exact_image| {
        if let Some(ref mut rotation) = exact_image.rotation {
            *rotation += 0.5 * time.delta_seconds();
        }
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(ExactImagePlugin)
        .add_startup_system(spawn_example)
        .add_system(rotate_images)
        .run();
}
