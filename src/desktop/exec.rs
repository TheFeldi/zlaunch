use crate::desktop::entry::DesktopEntry;
use crate::desktop::env::get_session_environment;
use std::process::Command;

pub fn launch_application(entry: &DesktopEntry) -> anyhow::Result<()> {
    let exec = clean_exec_string(&entry.exec);

    if entry.terminal {
        launch_in_terminal(&exec)?;
    } else {
        launch_detached(&exec)?;
    }

    Ok(())
}

fn clean_exec_string(exec: &str) -> String {
    let mut result = exec.to_string();

    for placeholder in [
        "%f", "%F", "%u", "%U", "%d", "%D", "%n", "%N", "%i", "%c", "%k",
    ] {
        result = result.replace(placeholder, "");
    }

    result.trim().to_string()
}

fn launch_detached(exec: &str) -> anyhow::Result<()> {
    let parts: Vec<&str> = exec.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty exec command");
    }

    let program = parts[0];
    let args = &parts[1..];

    Command::new(program)
        .args(args)
        .env_clear()
        .envs(get_session_environment().iter())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;

    Ok(())
}

fn launch_in_terminal(exec: &str) -> anyhow::Result<()> {
    let terminal = get_terminal()?;

    Command::new(&terminal)
        .arg("-e")
        .arg(exec)
        .env_clear()
        .envs(get_session_environment().iter())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;

    Ok(())
}

fn get_terminal() -> anyhow::Result<String> {
    if let Ok(terminal) = std::env::var("TERMINAL") {
        return Ok(terminal);
    }

    if Command::new("which")
        .arg("xterm")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        return Ok("xterm".to_string());
    }

    anyhow::bail!("No terminal emulator found. Set $TERMINAL environment variable.")
}
