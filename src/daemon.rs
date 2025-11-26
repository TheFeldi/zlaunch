use anyhow::Result;
use gpui::{Application, QuitMode, hsla};
use gpui_component::theme::{Theme, ThemeMode};

use crate::app::{DaemonEvent, WindowEvent, create_daemon_channel, window};
use crate::desktop::cache::load_applications;
use crate::desktop::capture_session_environment;
use crate::ipc::{Command, IpcServer, client};
use crate::items::ApplicationItem;
use crate::ui::init_launcher;

/// Run the launcher daemon.
/// This is the main entry point when no subcommand is provided.
pub fn run() -> Result<()> {
    // Capture the full session environment early, including from systemd user session.
    // This ensures launched applications get proper theming variables.
    capture_session_environment();

    let ipc_server = match IpcServer::new() {
        Ok(server) => server,
        Err(e) => {
            if client::is_daemon_running() {
                eprintln!("Daemon already running, sending toggle command...");
                client::send_command(Command::Toggle)?;
                return Ok(());
            }
            return Err(e);
        }
    };

    // Load applications and convert to ApplicationItems
    let entries = load_applications();
    let applications: Vec<ApplicationItem> = entries.into_iter().map(Into::into).collect();
    println!("Loaded {} applications", applications.len());

    // Create unified event channel
    let (event_tx, event_rx) = create_daemon_channel();

    // Spawn background thread for blocking IPC accept
    let ipc_listener = ipc_server.listener();
    let ipc_event_tx = event_tx.clone();
    std::thread::spawn(move || {
        loop {
            if let Some(cmd) = IpcServer::accept_blocking(&ipc_listener)
                && ipc_event_tx.send(DaemonEvent::Ipc(cmd)).is_err()
            {
                // Channel closed, exit thread
                break;
            }
        }
    });

    Application::new()
        .with_assets(gpui_component_assets::Assets)
        .with_quit_mode(QuitMode::Explicit)
        .run(move |cx| {
            gpui_component::init(cx);
            init_launcher(cx);
            Theme::change(ThemeMode::Dark, None, cx);

            // Customize theme for transparent background and no borders
            configure_theme(cx);

            let applications_clone = applications.clone();
            let mut window_handle = None;
            let mut visible = false;

            // Main event loop - async wait on channel, no polling needed
            cx.spawn(async move |cx: &mut gpui::AsyncApp| {
                while let Ok(event) = event_rx.recv_async().await {
                    match event {
                        DaemonEvent::Window(WindowEvent::RequestHide) if visible => {
                            let _ = cx.update(|cx| {
                                if let Some(ref handle) = window_handle {
                                    window::close_window(handle, cx);
                                }
                            });
                            window_handle = None;
                            visible = false;
                        }
                        DaemonEvent::Ipc(cmd) => {
                            let _ = cx.update(|cx| match cmd {
                                Command::Show | Command::Toggle if !visible => {
                                    match window::create_and_show_window(
                                        applications_clone.clone(),
                                        event_tx.clone(),
                                        cx,
                                    ) {
                                        Ok(handle) => {
                                            window_handle = Some(handle);
                                            visible = true;
                                        }
                                        Err(e) => eprintln!("Failed to create window: {}", e),
                                    }
                                }
                                Command::Hide | Command::Toggle if visible => {
                                    if let Some(ref handle) = window_handle {
                                        window::close_window(handle, cx);
                                        window_handle = None;
                                        visible = false;
                                    }
                                }
                                Command::Quit => {
                                    cx.quit();
                                }
                                _ => {}
                            });
                        }
                        _ => {}
                    }
                }
            })
            .detach();
        });

    Ok(())
}

/// Configure the global theme for transparent launcher appearance.
fn configure_theme(cx: &mut gpui::App) {
    let theme = Theme::global_mut(cx);
    theme.background = hsla(0.0, 0.0, 0.0, 0.0); // Fully transparent
    theme.window_border = hsla(0.0, 0.0, 0.0, 0.0); // No window border
    theme.border = hsla(0.0, 0.0, 1.0, 0.1); // Subtle separator between search and list
    theme.list_active_border = hsla(0.0, 0.0, 0.0, 0.0); // No selection border
    theme.list_active = hsla(0.0, 0.0, 0.0, 0.0); // Fully transparent - we handle selection ourselves
    theme.list_hover = hsla(0.0, 0.0, 0.0, 0.0); // Fully transparent - we handle hover ourselves
}
