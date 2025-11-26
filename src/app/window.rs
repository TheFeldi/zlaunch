use crate::app::{DaemonEvent, DaemonEventSender, WindowEvent};
use crate::items::ApplicationItem;
use crate::ui::LauncherView;
use gpui::{
    App, AppContext, Bounds, WindowBackgroundAppearance, WindowBounds, WindowDecorations,
    WindowHandle, WindowKind, WindowOptions,
    layer_shell::{Anchor, KeyboardInteractivity, Layer, LayerShellOptions},
    point, px, size,
};
use gpui_component::Root;

pub fn create_and_show_window(
    applications: Vec<ApplicationItem>,
    event_tx: DaemonEventSender,
    cx: &mut App,
) -> anyhow::Result<WindowHandle<Root>> {
    // Get display size - try displays() first, then primary_display(), then use huge fallback
    // The layer shell will clamp to actual screen size, so overshooting is fine
    let display_size = cx
        .displays()
        .first()
        .map(|d| d.bounds().size)
        .or_else(|| cx.primary_display().map(|d| d.bounds().size))
        .unwrap_or_else(|| size(px(7680.0), px(4320.0))); // 8K fallback - will be clamped

    let fullscreen_bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: display_size,
    };

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(fullscreen_bounds)),
        titlebar: None,
        focus: true,
        show: true,
        app_id: Some("zlaunch".to_string()),
        window_background: WindowBackgroundAppearance::Transparent,
        window_decorations: Some(WindowDecorations::Server),
        kind: WindowKind::LayerShell(LayerShellOptions {
            namespace: "zlaunch".to_string(),
            layer: Layer::Overlay,
            // Anchor to all edges = fullscreen overlay
            anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
            // Exclusive keyboard so typing works immediately
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            ..Default::default()
        }),
        ..Default::default()
    };

    let window_handle = cx.open_window(options, |window, cx| {
        let on_hide = move || {
            let _ = event_tx.send(DaemonEvent::Window(WindowEvent::RequestHide));
        };
        let view = cx.new(|cx| LauncherView::new(applications, on_hide, window, cx));

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
