use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::FocusPolicy;
use bevy::ui::RenderUiSystem;
use bevy::ui::UiStack;
use bevy::ui::UiSystem;

pub mod prelude {
    pub use crate::ExactSize;
    pub use crate::ExactImage;
    pub use crate::ExactImageBundle;
    pub use crate::ImageAlignment;
    pub use crate::ExactImagePlugin;
}

#[derive(Copy, Clone, Default, Reflect)]
pub enum ExactSize {
    #[default]
    /// The ui will attempt to size the node to preserve the aspect ratio of the image
    AttemptPreserveAspectRatio,
    /// The aspect ratio of the image will be preserved, regardless of the size of the node
    ForcePreserveAspectRatio,
    /// The image will be stretched to fill the ui node
    FillNode,
    /// Use the size of the source texture, regardless of the size of the node
    Texture,
    /// Use the size of the source texture scaled by the given factor, regardless of the size of the node
    Scaled(Vec2),
    /// Use a custom size, regardless of the size of the node
    Exactly(Vec2),
}

/// Alignment of the image within the node
#[derive(Copy, Clone, Default, Reflect)]
pub enum ImageAlignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    #[default]
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Anchor(Vec2),
}

#[derive(Component, Default, Reflect)]
pub struct ExactImage {
    pub texture: Handle<Image>,
    /// rotation of the image in radians
    pub rotation: Option<f32>,
    pub alignment: ImageAlignment,
    pub color: Color,
    pub size: ExactSize,
}

#[derive(Bundle)]
pub struct ExactImageBundle {
    /// The image to render
    pub image: ExactImage,
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `nodebundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl Default for ExactImageBundle {
    fn default() -> Self {
        ExactImageBundle {
            image: Default::default(),
            background_color: Color::NONE.into(),
            node: Default::default(),
            style: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

pub fn exact_image_system(
    mut commands: Commands,
    textures: Res<Assets<Image>>,
    images: Query<(Entity, &ExactImage), (Without<UiImage>, Without<Text>)>,
    mut calculated_sizes: Query<&mut CalculatedSize>,
) {
    for (id, image) in images.iter() {
        if let Some(texture) = textures.get(&image.texture) {
            match (image.size, calculated_sizes.get_mut(id)) {
                (
                    ExactSize::AttemptPreserveAspectRatio
                    | ExactSize::ForcePreserveAspectRatio,
                    Ok(mut calculated_size),
                ) => {
                    let texture_size = texture.size();
                    let size = Size::new(Val::Px(texture_size.x), Val::Px(texture_size.y));
                    if size != calculated_size.size {
                        calculated_size.size = size;
                    }
                }
                (
                    ExactSize::AttemptPreserveAspectRatio
                    | ExactSize::ForcePreserveAspectRatio,
                    Err(_),
                ) => {
                    let texture_size = texture.size();
                    let size = Size::new(Val::Px(texture_size.x), Val::Px(texture_size.y));
                    commands.entity(id).insert(CalculatedSize { size });
                }
                (_, Ok(_)) => {
                    commands.entity(id).remove::<CalculatedSize>();
                }
                _ => {}
            }
        }
    }
}

pub fn extract_exact_image(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    images: Extract<Res<Assets<Image>>>,
    ui_scale: Extract<Res<UiScale>>,
    ui_stack: Extract<Res<UiStack>>,
    uinode_query: Extract<
        Query<(
            &Node,
            &Style,
            &ExactImage,
            &GlobalTransform,
            &ComputedVisibility,
            Option<&CalculatedClip>,
        )>,
    >,
) {
    let scale_factor = ui_scale.scale as f32;
    for (stack_index, entity) in ui_stack.uinodes.iter().enumerate() {
        if let Ok((node, style, image, transform, visibility, clip)) = uinode_query.get(*entity) {
            if !visibility.is_visible() || image.color.a() == 0. || !images.contains(&image.texture)
            {
                continue;
            }
            let mut transform = transform.compute_matrix();

            let mut size = node.size();
            match image.size {
                ExactSize::ForcePreserveAspectRatio => {
                    if matches!(
                        style.flex_direction,
                        FlexDirection::Column | FlexDirection::ColumnReverse
                    ) {
                        size.x = size.y / images.get(&image.texture).unwrap().aspect_2d();
                    } else {
                        size.y = size.x * images.get(&image.texture).unwrap().aspect_2d();
                    }
                }
                ExactSize::Texture => {
                    size = images.get(&image.texture).unwrap().size() * scale_factor
                }
                ExactSize::Scaled(scale) => {
                    size = scale * images.get(&image.texture).unwrap().size() * scale_factor
                }
                ExactSize::Exactly(custom_size) => size = custom_size * scale_factor,
                _ => {}
            }

            use ImageAlignment::*;
            let alignment_offset = Vec2 {
                x: match image.alignment {
                    TopLeft | CenterLeft | BottomLeft => 0.5 * (-node.size().x + size.x),
                    TopCenter | Center | BottomCenter => 0.,
                    TopRight | CenterRight | BottomRight => 0.5 * (node.size().x - size.x),
                    Anchor(Vec2 { x, .. }) => x * node.size().x,
                },
                y: match image.alignment {
                    TopLeft | TopCenter | TopRight => 0.5 * (-node.size().y + size.y),
                    CenterLeft | Center | CenterRight => 0.,
                    BottomLeft | BottomCenter | BottomRight => 0.5 * (node.size().y - size.y),
                    Anchor(Vec2 { y, .. }) => y * 0.5 * node.size().y,
                },
            };

            transform = transform * Mat4::from_translation(alignment_offset.extend(0.));
            if let Some(rotation) = image.rotation {
                transform = transform * Mat4::from_rotation_z(rotation);
            }

            extracted_uinodes.uinodes.push(ExtractedUiNode {
                stack_index,
                transform,
                background_color: image.color,
                rect: Rect {
                    min: Vec2::ZERO,
                    max: size,
                },
                image: image.texture.clone_weak(),
                atlas_size: None,
                clip: clip.map(|clip| clip.clip),
                scale_factor,
            });
        }
    }
}

pub struct ExactImagePlugin;

impl Plugin for ExactImagePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ExactImage>().add_system_to_stage(
            CoreStage::PostUpdate,
            exact_image_system.before(UiSystem::Flex),
        );

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.add_system_to_stage(
            RenderStage::Extract,
            extract_exact_image.after(RenderUiSystem::ExtractNode),
        );
    }
}
