use std::sync::atomic::{AtomicBool, Ordering};

use gpui::{App, Hsla, Window, WindowAppearance, px, rgb};
use gpui_component::{Theme, ThemeMode};
use serde::{Deserialize, Serialize};

pub const NAV_WIDTH: f32 = 64.0;
pub const CONTEXT_WIDTH: f32 = 256.0;
pub const INSPECTOR_WIDTH: f32 = 320.0;

static DARK_MODE: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemePreference {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemePreference {
    pub fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }
}

pub fn install(preference: ThemePreference, cx: &mut App) {
    apply(preference, None, cx);
}

pub fn apply(preference: ThemePreference, window: Option<&mut Window>, cx: &mut App) {
    let mode = match preference {
        ThemePreference::System => appearance_mode(
            window
                .as_ref()
                .map(|window| window.appearance())
                .unwrap_or_else(|| cx.window_appearance()),
        ),
        ThemePreference::Light => ThemeMode::Light,
        ThemePreference::Dark => ThemeMode::Dark,
    };
    DARK_MODE.store(mode.is_dark(), Ordering::Release);
    Theme::change(mode, window, cx);
    configure_component_theme(cx);
}

pub fn is_dark() -> bool {
    DARK_MODE.load(Ordering::Acquire)
}

fn appearance_mode(appearance: WindowAppearance) -> ThemeMode {
    match appearance {
        WindowAppearance::Dark | WindowAppearance::VibrantDark => ThemeMode::Dark,
        WindowAppearance::Light | WindowAppearance::VibrantLight => ThemeMode::Light,
    }
}

fn configure_component_theme(cx: &mut App) {
    let dark = is_dark();
    let theme = Theme::global_mut(cx);

    theme.font_family = ui_family().into();
    theme.font_size = px(14.0);
    theme.mono_font_size = px(12.0);
    theme.radius = px(6.0);
    theme.radius_lg = px(8.0);
    theme.shadow = false;

    theme.colors.background = paper();
    theme.colors.foreground = ink();
    theme.colors.border = line();
    theme.colors.input = line_strong();
    theme.colors.ring = cobalt();
    theme.colors.caret = cobalt();
    theme.colors.selection = pale_cobalt();

    theme.colors.primary = cobalt();
    theme.colors.primary_hover = if dark {
        color(0x82a6ff)
    } else {
        color(0x3d78f5)
    };
    theme.colors.primary_active = if dark {
        color(0x638cff)
    } else {
        color(0x1d56cc)
    };
    theme.colors.primary_foreground = color(0xffffff);

    theme.colors.secondary = subtle_surface();
    theme.colors.secondary_hover = hover_surface();
    theme.colors.secondary_active = line_strong();
    theme.colors.secondary_foreground = ink();

    theme.colors.accent = pale_cobalt();
    theme.colors.accent_foreground = if dark { color(0xdce7ff) } else { cobalt_dark() };
    theme.colors.muted = subtle_surface();
    theme.colors.muted_foreground = graphite();
    theme.colors.popover = surface();
    theme.colors.popover_foreground = ink();

    theme.colors.link = cobalt();
    theme.colors.link_hover = if dark { color(0x89aaff) } else { cobalt_dark() };
    theme.colors.link_active = if dark {
        color(0x6f97ff)
    } else {
        color(0x174bb5)
    };

    theme.colors.success = signal_green();
    theme.colors.success_hover = color(0x2aad80);
    theme.colors.success_active = color(0x187f5d);
    theme.colors.success_foreground = if dark {
        color(0x17191d)
    } else {
        color(0xffffff)
    };
    theme.colors.warning = amber();
    theme.colors.warning_hover = color(0xe2a23a);
    theme.colors.warning_active = color(0xb97710);
    theme.colors.warning_foreground = if dark { color(0x17191d) } else { ink() };
    theme.colors.danger = if dark {
        color(0xe06767)
    } else {
        color(0xc53d3d)
    };
    theme.colors.danger_hover = if dark {
        color(0xee7777)
    } else {
        color(0xa92f2f)
    };
    theme.colors.danger_active = color(0x8c2626);
    theme.colors.danger_foreground = color(0xffffff);
    theme.colors.info = cobalt();
    theme.colors.info_hover = if dark { color(0x89aaff) } else { cobalt_dark() };
    theme.colors.info_active = color(0x174bb5);
    theme.colors.info_foreground = if dark {
        color(0x17191d)
    } else {
        color(0xffffff)
    };

    theme.colors.sidebar = nav();
    theme.colors.sidebar_foreground = if dark { color(0xd1d3d8) } else { ink() };
    theme.colors.sidebar_border = nav_border();
    theme.colors.sidebar_accent = subtle_surface();
    theme.colors.sidebar_accent_foreground = ink();
    theme.colors.sidebar_primary = cobalt();
    theme.colors.sidebar_primary_foreground = if dark {
        color(0x17191d)
    } else {
        color(0xffffff)
    };

    theme.colors.list = surface();
    theme.colors.list_head = subtle_surface();
    theme.colors.list_even = if dark {
        color(0x1e2126)
    } else {
        color(0xf8f8f6)
    };
    theme.colors.list_hover = hover_surface();
    theme.colors.list_active = pale_cobalt();
    theme.colors.list_active_border = cobalt();

    theme.colors.table = surface();
    theme.colors.table_head = subtle_surface();
    theme.colors.table_head_foreground = graphite();
    theme.colors.table_even = if dark {
        color(0x1e2126)
    } else {
        color(0xf8f8f6)
    };
    theme.colors.table_hover = hover_surface();
    theme.colors.table_active = pale_cobalt();
    theme.colors.table_active_border = cobalt();
    theme.colors.table_row_border = line();

    theme.colors.title_bar = nav();
    theme.colors.title_bar_border = nav_border();
    theme.colors.scrollbar = Hsla::transparent_black();
    theme.colors.scrollbar_thumb = if dark {
        color(0x535a66)
    } else {
        color(0xb8bcb5)
    };
    theme.colors.scrollbar_thumb_hover = graphite();
}

pub fn ui_family() -> &'static str {
    if cfg!(target_os = "linux") {
        "FreeSans"
    } else {
        ".SystemUIFont"
    }
}

pub fn mono_family() -> &'static str {
    if cfg!(target_os = "macos") {
        "Menlo"
    } else if cfg!(target_os = "windows") {
        "Consolas"
    } else if cfg!(target_os = "linux") {
        "FreeMono"
    } else {
        "DejaVu Sans Mono"
    }
}

pub fn color(value: u32) -> Hsla {
    rgb(value).into()
}

pub fn nav() -> Hsla {
    color(if is_dark() { 0x24262b } else { 0xf7f5f0 })
}

pub fn nav_border() -> Hsla {
    color(if is_dark() { 0x34363d } else { 0xe4e0d8 })
}

pub fn ink() -> Hsla {
    color(if is_dark() { 0xe8eaed } else { 0x17191d })
}

pub fn graphite() -> Hsla {
    color(if is_dark() { 0xa2a8b2 } else { 0x656a72 })
}

pub fn paper() -> Hsla {
    color(if is_dark() { 0x27282b } else { 0xfaf9f6 })
}

pub fn surface() -> Hsla {
    color(if is_dark() { 0x303136 } else { 0xffffff })
}

pub fn subtle_surface() -> Hsla {
    color(if is_dark() { 0x3a3b41 } else { 0xf1efe9 })
}

pub fn hover_surface() -> Hsla {
    color(if is_dark() { 0x42434a } else { 0xf2f0eb })
}

pub fn line() -> Hsla {
    color(if is_dark() { 0x46474d } else { 0xe5e1d8 })
}

pub fn line_strong() -> Hsla {
    color(if is_dark() { 0x5b5c64 } else { 0xd8d3c9 })
}

pub fn vermilion() -> Hsla {
    color(if is_dark() { 0xec735b } else { 0xd94c36 })
}

pub fn cobalt() -> Hsla {
    color(if is_dark() { 0x6f97ff } else { 0x246bfe })
}

pub fn cobalt_dark() -> Hsla {
    color(if is_dark() { 0x9ab5ff } else { 0x1d56cc })
}

pub fn pale_cobalt() -> Hsla {
    color(if is_dark() { 0x293753 } else { 0xe8efff })
}

pub fn signal_green() -> Hsla {
    color(if is_dark() { 0x42bd8f } else { 0x1f9d73 })
}

pub fn amber() -> Hsla {
    color(if is_dark() { 0xe4a948 } else { 0xd7911e })
}

pub fn error_background() -> Hsla {
    color(if is_dark() { 0x3a2325 } else { 0xfff4f2 })
}

pub fn error_border() -> Hsla {
    color(if is_dark() { 0x744044 } else { 0xe1b6b1 })
}

pub fn error_text() -> Hsla {
    color(if is_dark() { 0xffa5a0 } else { 0x9c3028 })
}

pub fn success_background() -> Hsla {
    color(if is_dark() { 0x20352c } else { 0xf1f6ef })
}

pub fn success_text() -> Hsla {
    color(if is_dark() { 0x89d8b5 } else { 0x285a38 })
}
