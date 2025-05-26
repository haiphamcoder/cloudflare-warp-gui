use crate::shell::run_shell;
use crate::status::WarpStatus;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

pub fn check_warp_status() -> WarpStatus {
    let output = Command::new("which").arg("warp-cli").output();

    if output.is_err() || !output.unwrap().status.success() {
        return WarpStatus::NotInstalled;
    }

    let output = Command::new("warp-cli").arg("status").output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if stdout.contains("Connected") {
            return WarpStatus::Connected;
        } else {
            return WarpStatus::Disconnected;
        }
    }

    WarpStatus::Error("Failed to check warp status".to_string())
}

pub fn install_warp() -> Result<(), String> {
    let key_cmd = "curl -fsSL https://pkg.cloudflareclient.com/pubkey.gpg | sudo gpg --yes --dearmor --output /usr/share/keyrings/cloudflare-warp-archive-keyring.gpg";
    let repo_cmd = "echo \"deb [arch=amd64 signed-by=/usr/share/keyrings/cloudflare-warp-archive-keyring.gpg] https://pkg.cloudflareclient.com/ $(lsb_release -cs) main\" | sudo tee /etc/apt/sources.list.d/cloudflare-client.list";
    let install_cmd = "sudo apt-get update && sudo apt-get install -y cloudflare-warp";

    if !run_shell(key_cmd) {
        return Err("Failed to add Cloudflare's GPG key".to_string());
    }

    if !run_shell(repo_cmd) {
        return Err("Failed to add Cloudflare's repository".to_string());
    }

    if !run_shell(install_cmd) {
        return Err("Failed to install Cloudflare Warp".to_string());
    }

    Ok(())
}

pub fn warp_register() -> Result<(), String> {
    let mut child = Command::new("warp-cli")
        .arg("registration")
        .arg("new")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start warp-cli: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        sleep(Duration::from_millis(300));
        let _ = stdin.write_all(b"y\n");
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to register warp: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("Old registration is still around") {
        let del_output = Command::new("warp-cli")
            .arg("registration")
            .arg("delete")
            .output()
            .map_err(|e| format!("Failed to delete old registration: {}", e))?;

        if !del_output.status.success() {
            return Err(format!(
                "Failed to delete old registration: {}",
                String::from_utf8_lossy(&del_output.stderr)
            ));
        }

        let mut child = Command::new("warp-cli")
            .arg("registration")
            .arg("new")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start warp-cli: {}", e))?;

        if let Some(stdin) = child.stdin.as_mut() {
            sleep(Duration::from_millis(300));
            let _ = stdin.write_all(b"y\n");
        }

        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed to register warp: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    } else if output.status.success() {
        Ok(())
    } else {
        Err(stderr.to_string())
    }
}

pub fn warp_connect() -> Result<(), String> {
    let output = Command::new("warp-cli").arg("connect").output();
    if let Ok(out) = output {
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    } else {
        Err("Failed to connect warp".to_string())
    }
}

pub fn warp_disconnect() -> Result<(), String> {
    let output = Command::new("warp-cli").arg("disconnect").output();
    if let Ok(out) = output {
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    } else {
        Err("Failed to disconnect warp".to_string())
    }
}
