use gpui::{App, Hsla, px, rgb};
use gpui_component::{Theme, ThemeMode};

pub const NAV_WIDTH: f32 = 72.0;
pub const CONTEXT_WIDTH: f32 = 272.0;
pub const INSPECTOR_WIDTH: f32 = 320.0;

pub fn install(cx: &mut App) {
    Theme::change(ThemeMode::Light, None, cx);
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
    theme.colors.primary_hover = color(0xc9432b);
    theme.colors.primary_active = color(0xad3522);
    theme.colors.primary_foreground = color(0xffffff);

    theme.colors.secondary = color(0xf0f1ee);
    theme.colors.secondary_hover = color(0xe4e6e1);
    theme.colors.secondary_active = color(0xd7dad4);
    theme.colors.secondary_foreground = ink();

    theme.colors.accent = pale_cobalt();
    theme.colors.accent_foreground = cobalt_dark();
    theme.colors.muted = color(0xe9ebe7);
    theme.colors.muted_foreground = graphite();
    theme.colors.popover = color(0xffffff);
    theme.colors.popover_foreground = ink();

    theme.colors.link = cobalt();
    theme.colors.link_hover = cobalt_dark();
    theme.colors.link_active = color(0x174bb5);

    theme.colors.success = signal_green();
    theme.colors.success_hover = color(0x187f5d);
    theme.colors.success_active = color(0x13664b);
    theme.colors.success_foreground = color(0xffffff);
    theme.colors.warning = amber();
    theme.colors.warning_hover = color(0xb97710);
    theme.colors.warning_active = color(0x965f0b);
    theme.colors.warning_foreground = ink();
    theme.colors.danger = color(0xc53d3d);
    theme.colors.danger_hover = color(0xa92f2f);
    theme.colors.danger_active = color(0x8c2626);
    theme.colors.danger_foreground = color(0xffffff);
    theme.colors.info = cobalt();
    theme.colors.info_hover = cobalt_dark();
    theme.colors.info_active = color(0x174bb5);
    theme.colors.info_foreground = color(0xffffff);

    theme.colors.sidebar = nav();
    theme.colors.sidebar_foreground = color(0xeef0ec);
    theme.colors.sidebar_border = color(0x30343b);
    theme.colors.sidebar_accent = color(0x292d33);
    theme.colors.sidebar_accent_foreground = color(0xffffff);
    theme.colors.sidebar_primary = vermilion();
    theme.colors.sidebar_primary_foreground = color(0xffffff);

    theme.colors.list = color(0xffffff);
    theme.colors.list_head = color(0xf0f1ee);
    theme.colors.list_even = color(0xf8f8f6);
    theme.colors.list_hover = color(0xf1f4fb);
    theme.colors.list_active = pale_cobalt();
    theme.colors.list_active_border = cobalt();

    theme.colors.table = color(0xffffff);
    theme.colors.table_head = color(0xf0f1ee);
    theme.colors.table_head_foreground = graphite();
    theme.colors.table_even = color(0xf8f8f6);
    theme.colors.table_hover = color(0xf1f4fb);
    theme.colors.table_active = pale_cobalt();
    theme.colors.table_active_border = cobalt();
    theme.colors.table_row_border = line();

    theme.colors.title_bar = nav();
    theme.colors.title_bar_border = color(0x30343b);
    theme.colors.scrollbar = Hsla::transparent_black();
    theme.colors.scrollbar_thumb = color(0xb8bcb5);
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
    color(0x17191d)
}

pub fn ink() -> Hsla {
    color(0x17191d)
}

pub fn graphite() -> Hsla {
    color(0x656a72)
}

pub fn paper() -> Hsla {
    color(0xf6f7f4)
}

pub fn surface() -> Hsla {
    color(0xffffff)
}

pub fn line() -> Hsla {
    color(0xdfe1dc)
}

pub fn line_strong() -> Hsla {
    color(0xc8cbc5)
}

pub fn vermilion() -> Hsla {
    color(0xe65335)
}

pub fn cobalt() -> Hsla {
    color(0x246bfe)
}

pub fn cobalt_dark() -> Hsla {
    color(0x1d56cc)
}

pub fn pale_cobalt() -> Hsla {
    color(0xe8efff)
}

pub fn signal_green() -> Hsla {
    color(0x1f9d73)
}

pub fn amber() -> Hsla {
    color(0xd7911e)
}
