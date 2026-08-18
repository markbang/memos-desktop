mod api;
mod config;
mod demo;
mod theme;
mod ui;

use gpui::{App, AppContext as _, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component::Root;
use gpui_component_assets::Assets;
use ui::MemosDesktop;

fn main() {
    let demo_mode = std::env::args().any(|argument| argument == "--demo");
    let app = Application::new().with_assets(Assets);

    app.run(move |cx: &mut App| {
        gpui_component::init(cx);
        theme::install(cx);

        let bounds = Bounds::centered(None, size(px(1440.0), px(900.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(1040.0), px(680.0))),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            let window = cx.open_window(options, |window, cx| {
                let app = cx.new(|cx| MemosDesktop::new(demo_mode, window, cx));
                cx.new(|cx| Root::new(app, window, cx))
            })?;

            window.update(cx, |_, window, _| {
                window.set_window_title("Memos Desktop");
                window.activate_window();
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
