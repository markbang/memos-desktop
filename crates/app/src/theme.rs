use std::sync::atomic::{AtomicBool, Ordering};

use gpui::{App, Hsla, Window, WindowAppearance, px, rgb};
use gpui_component::{Theme, ThemeMode};
use serde::{Deserialize, Serialize};

pub const NAV_WIDTH: f32 = 72.0;
pub const CONTEXT_WIDTH: f32 = 272.0;
pub const INSPECTOR_WIDTH: f32 = 360.0;

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
    theme.font_size = px(15.0);
    theme.mono_font_size = px(12.0);
    theme.radius = px(4.0);
    theme.radius_lg = px(6.0);
    theme.shadow = false;

    theme.colors.background = paper();
    theme.colors.foreground = ink();
    theme.colors.border = line();
    theme.colors.input = line_strong();
    theme.colors.ring = cobalt();
    theme.colors.caret = cobalt();
    theme.colors.selection = pale_cobalt();

    theme.colors.primary = vermilion();
    theme.colors.primary_hover = if dark {
        color(0xf4775e)
    } else {
        color(0xc9432b)
    };
    theme.colors.primary_active = if dark {
        color(0xd75a43)
    } else {
        color(0xad3522)
    };
    theme.colors.primary_foreground = color(0xffffff);

    theme.colors.secondary = if dark {
        color(0x2b2f36)
    } else {
        color(0xf0f1ee)
    };
    theme.colors.secondary_hover = if dark {
        color(0x363b44)
    } else {
        color(0xe4e6e1)
    };
    theme.colors.secondary_active = if dark {
        color(0x414751)
    } else {
        color(0xd7dad4)
    };
    theme.colors.secondary_foreground = ink();

    theme.colors.accent = pale_cobalt();
    theme.colors.accent_foreground = if dark { color(0xdce7ff) } else { cobalt_dark() };
    theme.colors.muted = if dark {
        color(0x2a2d33)
    } else {
        color(0xe9ebe7)
    };
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
    theme.colors.success_foreground = color(0xffffff);
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
    theme.colors.info_foreground = color(0xffffff);

    theme.colors.sidebar = nav();
    theme.colors.sidebar_foreground = color(0xeef0ec);
    theme.colors.sidebar_border = nav_border();
    theme.colors.sidebar_accent = color(0x292d33);
    theme.colors.sidebar_accent_foreground = color(0xffffff);
    theme.colors.sidebar_primary = vermilion();
    theme.colors.sidebar_primary_foreground = color(0xffffff);

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
    color(if is_dark() { 0x111317 } else { 0x17191d })
}

pub fn nav_border() -> Hsla {
    color(if is_dark() { 0x2b2f36 } else { 0x30343b })
}

pub fn ink() -> Hsla {
    color(if is_dark() { 0xe8eaed } else { 0x17191d })
}

pub fn graphite() -> Hsla {
    color(if is_dark() { 0xa2a8b2 } else { 0x656a72 })
}

pub fn paper() -> Hsla {
    color(if is_dark() { 0x181a1f } else { 0xf6f7f4 })
}

pub fn surface() -> Hsla {
    color(if is_dark() { 0x202329 } else { 0xffffff })
}

pub fn subtle_surface() -> Hsla {
    color(if is_dark() { 0x25282e } else { 0xf0f1ee })
}

pub fn hover_surface() -> Hsla {
    color(if is_dark() { 0x292d34 } else { 0xf1f4fb })
}

pub fn line() -> Hsla {
    color(if is_dark() { 0x343840 } else { 0xdfe1dc })
}

pub fn line_strong() -> Hsla {
    color(if is_dark() { 0x484e58 } else { 0xc8cbc5 })
}

pub fn vermilion() -> Hsla {
    color(if is_dark() { 0xea684f } else { 0xe65335 })
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
