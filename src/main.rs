use anyhow::Result;
use clap::{Parser, Subcommand};
use gpui::hsla;
use gpui::{Application, QuitMode, Timer};
use gpui_component::theme::{Theme, ThemeMode};
use std::time::Duration;
use zlaunch::app::{WindowEvent, create_event_channel, window};
use zlaunch::desktop::cache::load_applications;
use zlaunch::desktop::capture_session_environment;
use zlaunch::ipc::{Command, IpcServer, client};
use zlaunch::ui::init_launcher;

#[derive(Parser)]
#[command(name = "zlaunch")]
#[command(about = "A fast application launcher for Linux")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the launcher window
    Show,
    /// Hide the launcher window
    Hide,
    /// Toggle the launcher window visibility
    Toggle,
    /// Quit the daemon
    Quit,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => handle_client_command(cmd),
        None => run_daemon(),
    }
}

fn handle_client_command(cmd: Commands) -> Result<()> {
    let ipc_cmd = match cmd {
        Commands::Show => Command::Show,
        Commands::Hide => Command::Hide,
        Commands::Toggle => Command::Toggle,
        Commands::Quit => Command::Quit,
    };

    if !client::is_daemon_running() {
        eprintln!("Error: zlaunch daemon is not running");
        eprintln!("Start the daemon first by running: zlaunch");
        std::process::exit(1);
    }

    client::send_command(ipc_cmd)?;
    Ok(())
}

fn run_daemon() -> Result<()> {
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

    let entries = load_applications();
    println!("Loaded {} applications", entries.len());

    Application::new()
        .with_assets(gpui_component_assets::Assets)
        .with_quit_mode(QuitMode::Explicit)
        .run(move |cx| {
            gpui_component::init(cx);
            init_launcher(cx);
            Theme::change(ThemeMode::Dark, None, cx);

            // Customize theme for transparent background and no borders
            {
                let theme = Theme::global_mut(cx);
                theme.background = hsla(0.0, 0.0, 0.0, 0.0); // Fully transparent
                theme.window_border = hsla(0.0, 0.0, 0.0, 0.0); // No window border
                theme.border = hsla(0.0, 0.0, 1.0, 0.1); // Subtle separator between search and list
                theme.list_active_border = hsla(0.0, 0.0, 0.0, 0.0); // No selection border
                theme.list_active = hsla(0.0, 0.0, 0.0, 0.0); // Fully transparent - we handle selection ourselves
                theme.list_hover = hsla(0.0, 0.0, 0.0, 0.0); // Fully transparent - we handle hover ourselves
            }

            let entries_clone = entries.clone();
            let (event_tx, event_rx) = create_event_channel();
            let mut window_handle = None;
            let mut visible = false;

            cx.spawn(async move |cx: &mut gpui::AsyncApp| {
                loop {
                    Timer::after(Duration::from_millis(50)).await;

                    // Poll for window events (ESC pressed, app launched)
                    if let Ok(WindowEvent::RequestHide) = event_rx.try_recv() {
                        if visible {
                            let _ = cx.update(|cx| {
                                if let Some(ref handle) = window_handle {
                                    window::close_window(handle, cx);
                                }
                            });
                            window_handle = None;
                            visible = false;
                        }
                    }

                    // Poll for IPC commands
                    if let Some(cmd) = ipc_server.poll_command() {
                        let _ = cx.update(|cx| match cmd {
                            Command::Show | Command::Toggle if !visible => {
                                match window::create_and_show_window(
                                    entries_clone.clone(),
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
                }
            })
            .detach();
        });

    Ok(())
}
