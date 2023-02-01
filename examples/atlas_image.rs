use bevy::prelude::*;
use bevy_ui_exact_image::prelude::*;

fn main() {
    App::new()
        .insert_resource(UiScale { scale: 0.5 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1380.0 / 2.0,
                height: 276.0 / 2.0,
                resizable: false,
                ..Default::default()
            },
             ..Default::default()
        })
        )
        .add_plugin(ExactImagePlugin)
        .add_startup_system(spawn_example)
        .run();
}

fn spawn_example(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_atlas_image = asset_server.load("orientation_big.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image.clone(), Vec2::splat(128.), 2, 2, None, None);
    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

   commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(1380.), Val::Px(276.)),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        }
   ).with_children(|builder| {
        builder.spawn(ExactImageBundle {
            image: ExactImage {
                texture: texture_atlas_image,
                color: Color::WHITE,
                size: ExactSize::Texture,
                ..Default::default()
            },
            style: Style {
                size: Size::new(Val::Px(256.), Val::Px(256.)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::RED),
            ..Default::default()
        });

        for (index, alignment) in [ImageAlignment::TopLeft, ImageAlignment::TopRight, ImageAlignment::BottomLeft, ImageAlignment::BottomRight].into_iter().enumerate() {
            builder.spawn(ExactAtlasImageBundle {
                image: ExactAtlasImage {
                    atlas: texture_atlas_handle.clone(),
                    index,
                    color: Color::WHITE,
                    size: ExactSize::Texture,
                    alignment,
                    rotation: None,
                },
                style: Style {
                    size: Size::new(Val::Px(256.), Val::Px(256.)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            });
        }
    });
}

