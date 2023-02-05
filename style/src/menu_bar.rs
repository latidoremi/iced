//! Change the appearance of menu bars and their menus.
use iced_core::{Color};

/// The appearance of a menu bar and it's menus.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The [`Background`] of the menu bar and it's menus.
    pub background: Color,
    /// The border width of the menu bar and it's menus.
    pub border_width: f32,
    /// The border radius of the menu bar and it's menus.
    pub border_radius: f32,
    /// The border [`Color`] of the menu bar and it's menus.
    pub border_color: Color,
    /// The highlighted path [`Color`] of the the menu bar and it's menus.
    /// If you'd like to customize this, bear in mind that highlighted path
    /// is drawn on top of the previous menu, you would usually want a 
    /// somewaht transparent color
    pub highlight_path: Color,
}
impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            background: Color::from([0.85;3]),
            border_width: 1.0,
            border_radius: 4.0,
            border_color: Color::from([0.5;3]),
            highlight_path: Color::from([0.0, 0.0, 0.0, 0.3]),
        }
    }
}

/// The style sheet of a menu bar and it's menus.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of a menu bar and it's menus.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}
