//! Change the appearance of menu bars and their menus.
use iced_core::{Color};

/// The appearance of a menu bar and its menus.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The background color of the menu bar and its menus.
    pub background: Color,
    /// The border width of the menu bar and its menus.
    pub border_width: f32,
    /// The border radius of the menu bar and its menus.
    pub border_radius: [f32;4],
    /// The border [`Color`] of the menu bar and its menus.
    pub border_color: Color,
    /// The expand value of the menus' background
    pub background_expand: [u16;4],
    /// The highlighted path [`Color`] of the the menu bar and its menus.
    pub path: Color,
}
impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            background: Color::from([0.85;3]),
            border_width: 1.0,
            border_radius: [4.0;4],
            border_color: Color::from([0.5;3]),
            background_expand: [4;4],
            path: Color::from([0.0, 0.0, 0.0, 0.3]),
        }
    }
}

/// The style sheet of a menu bar and its menus.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of a menu bar and its menus.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}
