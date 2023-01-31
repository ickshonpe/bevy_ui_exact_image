# bevy_ui_exact_image
[![crates.io](https://img.shields.io/crates/v/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_ui_exact_image)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_ui_exact_image)
[![crates.io](https://img.shields.io/crates/d/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_ui_exact_image)

* Forces Bevy UI to draw images in whatever sizes you want.

* Preserve the aspect ratio of images, regardless of the UI layout.

* Also supports image rotation. 




### Examples

```
cargo --run --example minimal
cargo --run --example rotation
cargo --run --example size
cargo --run --example alignment


```

### Limitations

* No image flipping. Not possible (at least without a lot of work) with a third party implementation atm in Bevy 0.9.

* No texture atlas support. Isn't difficult to add, will 
