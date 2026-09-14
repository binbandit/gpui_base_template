//! Semantic design tokens. Views consume roles such as `surface` and
//! `accent`, never raw colors, so themes stay coherent as the app grows.

use gpui::{App, Global, Hsla, ReadGlobal, UpdateGlobal, rgb};
use serde::{Deserialize, Serialize};

/// User-selectable appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub const fn toggled(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

/// Process-wide semantic palette. Theme is global because overlays and future
/// windows must resolve the same tokens; page and domain state remain entities.
#[derive(Debug, Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    pub canvas: Hsla,
    pub sidebar: Hsla,
    pub surface: Hsla,
    pub surface_hover: Hsla,
    pub surface_muted: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_faint: Hsla,
    pub border: Hsla,
    pub border_strong: Hsla,
    pub focus_ring: Hsla,
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_ink: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,
}

impl Global for Theme {}

impl Theme {
    pub fn install(mode: ThemeMode, cx: &mut App) {
        Self::set_global(cx, Self::from_mode(mode));
    }

    pub fn current(cx: &App) -> Self {
        Self::global(cx).clone()
    }

    pub fn set_mode(mode: ThemeMode, cx: &mut App) {
        Self::set_global(cx, Self::from_mode(mode));
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self {
                mode,
                canvas: color(0x16181D),
                sidebar: color(0x121419),
                surface: color(0x1D2027),
                surface_hover: color(0x282D37),
                surface_muted: color(0x191C23),
                text: color(0xEDF0F5),
                text_muted: color(0xABB3C2),
                text_faint: color(0x929DAF),
                border: color(0x2C323E),
                border_strong: color(0x414B5C),
                focus_ring: color(0x90B4FF),
                accent: color(0x90B4FF),
                accent_hover: color(0xADC8FF),
                accent_ink: color(0x101D37),
                success: color(0x85D4AB),
                warning: color(0xE8C17A),
                danger: color(0xF89B9B),
            },
            ThemeMode::Light => Self {
                mode,
                canvas: color(0xF8F9FC),
                sidebar: color(0xF0F2F6),
                surface: color(0xFFFFFF),
                surface_hover: color(0xE9EDF5),
                surface_muted: color(0xF4F6FA),
                text: color(0x1B2433),
                text_muted: color(0x586579),
                text_faint: color(0x5C687B),
                border: color(0xDFE4ED),
                border_strong: color(0xC2CBD9),
                focus_ring: color(0x265DCC),
                accent: color(0x265DCC),
                accent_hover: color(0x1D4FAF),
                accent_ink: color(0xFFFFFF),
                success: color(0x26734A),
                warning: color(0x81520D),
                danger: color(0xAD3D47),
            },
        }
    }
}

fn color(hex: u32) -> Hsla {
    rgb(hex).into()
}

#[cfg(test)]
mod tests {
    use super::{Theme, ThemeMode};
    use gpui::{Hsla, Rgba};

    fn relative_luminance(color: Hsla) -> f32 {
        let rgba = Rgba::from(color);
        let linear = |channel: f32| {
            if channel <= 0.04045 {
                channel / 12.92
            } else {
                ((channel + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(rgba.r) + 0.7152 * linear(rgba.g) + 0.0722 * linear(rgba.b)
    }

    fn contrast(a: Hsla, b: Hsla) -> f32 {
        let (lighter, darker) = {
            let a = relative_luminance(a);
            let b = relative_luminance(b);
            if a > b { (a, b) } else { (b, a) }
        };
        (lighter + 0.05) / (darker + 0.05)
    }

    #[test]
    fn toggling_is_reversible() {
        assert_eq!(ThemeMode::Dark.toggled(), ThemeMode::Light);
        assert_eq!(ThemeMode::Dark.toggled().toggled(), ThemeMode::Dark);
    }

    #[test]
    fn themes_preserve_the_requested_mode() {
        assert_eq!(Theme::from_mode(ThemeMode::Light).mode, ThemeMode::Light);
        assert_eq!(Theme::from_mode(ThemeMode::Dark).mode, ThemeMode::Dark);
    }

    #[test]
    fn normal_text_pairs_meet_wcag_aa_contrast() {
        for theme in [
            Theme::from_mode(ThemeMode::Light),
            Theme::from_mode(ThemeMode::Dark),
        ] {
            for (foreground, background) in [
                (theme.text, theme.surface),
                (theme.text_muted, theme.surface),
                (theme.text_faint, theme.surface),
                (theme.text_faint, theme.sidebar),
                (theme.accent, theme.surface),
                (theme.accent_ink, theme.accent),
                (theme.success, theme.surface),
                (theme.warning, theme.surface),
                (theme.danger, theme.surface),
            ] {
                let ratio = contrast(foreground, background);
                assert!(
                    ratio >= 4.5,
                    "foreground/background contrast was {ratio:.2}:1"
                );
            }
            assert!(contrast(theme.focus_ring, theme.surface) >= 3.0);
        }
    }

    #[gpui::test]
    fn theme_can_be_installed_in_gpui(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            Theme::install(ThemeMode::Light, cx);
            assert_eq!(Theme::current(cx).mode, ThemeMode::Light);
        });
    }
}
