# bevy_ui_exact_image
[![crates.io](https://img.shields.io/crates/v/bevy_ui_exact_image)](https://crates.io/crates/bevy_ui_exact_image)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_ui_exact_image)
[![crates.io](https://img.shields.io/crates/d/bevy_ui_exact_image)](https://crates.io/crates/bevy_ui_exact_image)

* Force the Bevy UI to draw images in whatever sizes you want.
* Preserve the aspect ratio of images, regardless of the UI layout.
* Also supports image rotation. 

![image](/assets/sizes.png)

![image](/assets/rotation.png)

#
## Usage

Add the dependency to your bevy project:

```
cargo add bevy_ui_exact_image
```

Then to draw a sized image within a Bevy UI node:

```rust
use bevy::prelude::*;
use bevy_ui_exact_image::prelude::*;

fn spawn_example(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((ExactImageBundle {
        image: ExactImage {
            texture: assets.load("orientation.png"),
            // exact images have their own color, independent from the background color.
            color: Color::WHITE,
            // force the UI to display the texture at 300 x 200 size
            size: ExactSize::Exactly(Vec2::new(300., 200.)),
            // sets the alignment of the image if it doesn't fill the containing node
            alignment: ImageAlignment::BottomCenter,
            // use Some(rads) to set rotation
            rotation: None,
        },
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Px(400.0)),
            ..Default::default()
        },
        /// give the containing node a red color
        background_color: BackgroundColor(Color::RED),
        ..Default::default()
    },));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(ExactImagePlugin)
        .add_startup_system(spawn_example)
        .run();
}
```

Result:

![image](/assets/example.png)

#
## Examples

```
cargo --run --example minimal
cargo --run --example rotation
cargo --run --example size
cargo --run --example alignment
```

## Limitations

* No image flipping. Not possible (at least not trivially) with a third party implementation atm in Bevy 0.9.

* No texture atlas support. Will add this later.

## Notes

Name stolen from inodentry's related Bevy issue #7439

[https://github.com/bevyengine/bevy/issues/7349](https://github.com/bevyengine/bevy/issues/7349)