use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use vpn_core::wireguard::WireGuardConfig;

const INTERFACE_NAME: &str = "wg0";

pub fn apply_config(config: &WireGuardConfig) -> Result<()> {
    let config_path = create_config_file(config)?;

    #[cfg(target_os = "linux")]
    {
        start_tunnel_linux(&config_path)?;
    }

    #[cfg(target_os = "windows")]
    {
        start_tunnel_windows(&config_path)?;
    }

    #[cfg(target_os = "macos")]
    {
        start_tunnel_macos(&config_path)?;
    }

    Ok(())
}

pub fn stop_tunnel() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        stop_tunnel_linux()?;
    }

    #[cfg(target_os = "windows")]
    {
        stop_tunnel_windows()?;
    }

    #[cfg(target_os = "macos")]
    {
        stop_tunnel_macos()?;
    }

    let config_path = get_config_path();
    if config_path.exists() {
        fs::remove_file(config_path).ok();
    }

    Ok(())
}

fn create_config_file(config: &WireGuardConfig) -> Result<PathBuf> {
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config_content = config.to_ini();
    fs::write(&config_path, config_content).context("Failed to write WireGuard config file")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&config_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&config_path, perms)?;
    }

    Ok(config_path)
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("vpn-client");
    path.push(format!("{}.conf", INTERFACE_NAME));
    path
}

#[cfg(target_os = "linux")]
fn start_tunnel_linux(config_path: &PathBuf) -> Result<()> {
    let output = Command::new("sudo")
        .arg("wg-quick")
        .arg("up")
        .arg(config_path)
        .output()
        .context("Failed to execute wg-quick up (sudo required)")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("wg-quick up failed: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn stop_tunnel_linux() -> Result<()> {
    let config_path = get_config_path();

    let output = Command::new("sudo")
        .arg("wg-quick")
        .arg("down")
        .arg(config_path)
        .output()
        .context("Failed to execute wg-quick down")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("wg-quick down warning: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn start_tunnel_windows(config_path: &PathBuf) -> Result<()> {
    let output = Command::new("wireguard")
        .arg("/installtunnelservice")
        .arg(config_path)
        .output()
        .context("Failed to start WireGuard tunnel (WireGuard for Windows required)")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("WireGuard start failed: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn stop_tunnel_windows() -> Result<()> {
    let output = Command::new("wireguard")
        .arg("/uninstalltunnelservice")
        .arg(INTERFACE_NAME)
        .output()
        .context("Failed to stop WireGuard tunnel")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("WireGuard stop warning: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn start_tunnel_macos(config_path: &PathBuf) -> Result<()> {
    let output = Command::new("sudo")
        .arg("wg-quick")
        .arg("up")
        .arg(config_path)
        .output()
        .context("Failed to execute wg-quick up (install WireGuard via Homebrew)")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("wg-quick up failed: {}", stderr);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn stop_tunnel_macos() -> Result<()> {
    let config_path = get_config_path();

    let output = Command::new("sudo")
        .arg("wg-quick")
        .arg("down")
        .arg(config_path)
        .output()
        .context("Failed to execute wg-quick down")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("wg-quick down warning: {}", stderr);
    }

    Ok(())
}
