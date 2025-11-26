use crate::app::{EventSender, WindowEvent};
use crate::desktop::DesktopEntry;
use crate::ui::LauncherView;
use gpui::{
    App, AppContext, Bounds, WindowBackgroundAppearance, WindowBounds, WindowDecorations,
    WindowHandle, WindowKind, WindowOptions,
    layer_shell::{Anchor, KeyboardInteractivity, Layer, LayerShellOptions},
    point, px, size,
};
use gpui_component::Root;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 400.0;

pub fn create_and_show_window(
    entries: Vec<DesktopEntry>,
    event_tx: EventSender,
    cx: &mut App,
) -> anyhow::Result<WindowHandle<Root>> {
    let bounds = calculate_centered_bounds(cx);

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: None,
        focus: true,
        show: true,
        app_id: Some("zlaunch".to_string()),
        window_background: WindowBackgroundAppearance::Blurred,
        window_decorations: Some(WindowDecorations::Server), // Avoid client-side shadow
        kind: WindowKind::LayerShell(LayerShellOptions {
            namespace: "zlaunch".to_string(),
            layer: Layer::Overlay,
            anchor: Anchor::empty(),
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            ..Default::default()
        }),
        ..Default::default()
    };

    let window_handle = cx.open_window(options, |window, cx| {
        // Try to eliminate client-side decoration padding
        window.set_client_inset(px(0.0));
        let on_hide = move || {
            let _ = event_tx.send(WindowEvent::RequestHide);
        };
        let view = cx.new(|cx| LauncherView::new(entries, on_hide, window, cx));

        // Auto-focus the list/search input
        view.update(cx, |launcher: &mut LauncherView, cx| {
            launcher.focus(window, cx);
        });

        cx.new(|cx| Root::new(view, window, cx))
    })?;

    window_handle.update(cx, |_root, window, _cx| {
        window.activate_window();
    })?;

    Ok(window_handle)
}

pub fn close_window(handle: &WindowHandle<Root>, cx: &mut App) {
    let _ = handle.update(cx, |_root, window, _cx| {
        window.remove_window();
    });
}

fn calculate_centered_bounds(cx: &App) -> Bounds<gpui::Pixels> {
    let display_size = cx
        .primary_display()
        .map(|d| d.bounds().size)
        .unwrap_or_else(|| size(px(1920.0), px(1080.0)));

    let x = (display_size.width - px(WINDOW_WIDTH)) / 2.0;
    let y = (display_size.height - px(WINDOW_HEIGHT)) / 3.0;

    Bounds {
        origin: point(x, y),
        size: size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)),
    }
}
