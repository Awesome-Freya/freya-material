#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::unreadable_literal,
    clippy::derive_partial_eq_without_eq,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

mod components;
pub mod material_design;

pub mod prelude {
    pub use crate::{
        components::*,
        material_design::{
            motion::{Easing, EasingDuration},
            Elevation, Shape, TypescaleSize, TypescaleVariant,
        },
        set_material_theme, use_material_theme, ArgbExt, LaunchConfigExt,
    };
}

use freya::prelude::{try_use_context, use_context_provider, LaunchConfig, Signal, Writable};
use material_colors::{color::Argb, scheme::Scheme, theme::ThemeBuilder};

static ROBOTO_THIN: &[u8] = include_bytes!("../assets/Roboto-Thin.ttf");
static ROBOTO_THIN_ITALIC: &[u8] = include_bytes!("../assets/Roboto-ThinItalic.ttf");
static ROBOTO_LIGHT: &[u8] = include_bytes!("../assets/Roboto-Light.ttf");
static ROBOTO_LIGHT_ITALIC: &[u8] = include_bytes!("../assets/Roboto-LightItalic.ttf");
static ROBOTO_REGULAR: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");
static ROBOTO_ITALIC: &[u8] = include_bytes!("../assets/Roboto-Italic.ttf");
static ROBOTO_MEDIUM: &[u8] = include_bytes!("../assets/Roboto-Medium.ttf");
static ROBOTO_MEDIUM_ITALIC: &[u8] = include_bytes!("../assets/Roboto-MediumItalic.ttf");
static ROBOTO_BOLD: &[u8] = include_bytes!("../assets/Roboto-Bold.ttf");
static ROBOTO_BOLD_ITALIC: &[u8] = include_bytes!("../assets/Roboto-BoldItalic.ttf");
static ROBOTO_BLACK: &[u8] = include_bytes!("../assets/Roboto-Black.ttf");
static ROBOTO_BLACK_ITALIC: &[u8] = include_bytes!("../assets/Roboto-BlackItalic.ttf");

pub trait LaunchConfigExt {
    #[must_use]
    fn with_roboto(self) -> Self;
}

impl<T: Clone> LaunchConfigExt for LaunchConfig<'_, T> {
    fn with_roboto(self) -> Self {
        self.with_font("Roboto", ROBOTO_THIN)
            .with_font("Roboto", ROBOTO_THIN_ITALIC)
            .with_font("Roboto", ROBOTO_LIGHT)
            .with_font("Roboto", ROBOTO_LIGHT_ITALIC)
            .with_font("Roboto", ROBOTO_REGULAR)
            .with_font("Roboto", ROBOTO_ITALIC)
            .with_font("Roboto", ROBOTO_MEDIUM)
            .with_font("Roboto", ROBOTO_MEDIUM_ITALIC)
            .with_font("Roboto", ROBOTO_BOLD)
            .with_font("Roboto", ROBOTO_BOLD_ITALIC)
            .with_font("Roboto", ROBOTO_BLACK)
            .with_font("Roboto", ROBOTO_BLACK_ITALIC)
    }
}

pub trait ArgbExt {
    fn as_rgba(&self) -> String;
    #[must_use]
    fn with_alpha_f32(self, alpha: f32) -> Self;
}

impl ArgbExt for Argb {
    fn as_rgba(&self) -> String {
        format!(
            "rgb({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }

    fn with_alpha_f32(mut self, alpha: f32) -> Self {
        self.alpha = (255.0 * alpha) as u8;

        self
    }
}

#[must_use]
pub fn use_material_theme() -> Signal<Scheme> {
    try_use_context::<Signal<Scheme>>().map_or_else(
        || {
            use_context_provider(|| {
                Signal::new(
                    ThemeBuilder::with_source(Argb::from_u32(0xFFBC0D))
                        .build()
                        .schemes
                        .dark,
                )
            })
        },
        |value| value,
    )
}

pub fn set_material_theme(scheme: Scheme) {
    *use_material_theme().write() = scheme;
}
