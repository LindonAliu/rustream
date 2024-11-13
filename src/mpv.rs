use crate::m3u::Channel;
use std::io::BufRead;
use std::process::Stdio;
use std::{env::consts::OS, path::Path};
use which::which;

pub fn play(channel: Channel) -> Option<()> {
    let path = get_mpv_path();
    let args = get_play_args(&channel, path.clone()).unwrap();
    let mut cmd = match std::process::Command::new(path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to spawn mpv process: {}", e);
            return None;
        }
    };

    let status = cmd.wait().unwrap();

    if !status.success() {
        if let Some(stdout) = cmd.stdout.take() {
            let mut error = String::new();
            let mut lines = std::io::BufReader::new(stdout).lines();
            let mut first = true;
            while let Some(line) = lines.next() {
                error += &line.unwrap();
                if !first {
                    error += "\n";
                } else {
                    first = false;
                }
            }
            if !error.is_empty() {
                eprintln!("{}", error);
            } else {
                eprintln!("Mpv encountered an unknown error");
            }
        }
    }
    return Some(());
}

fn get_play_args(channel: &Channel, path: String) -> Result<Vec<String>, std::io::Error> {
    let mut args = vec![channel.url.clone()];

    if channel.url.ends_with(".mkv") || channel.url.ends_with(".mp4") {
        args.push("--save-position-on-quit".to_string());
    }
    if OS == "macos" && path != "mpv" {
        args.push(
            "--script-opts=ytdl_hook-ytdl_path=".to_string()
                + find_macos_bin("yt-dlp".to_string()).as_str(),
        );
    }
    args.push(format!("--title={}", channel.name));
    args.push("--msg-level=all=error".to_string());

    Ok(args)
}

fn get_mpv_path() -> String {
    if OS == "linux" || which("mpv").is_ok() {
        return "mpv".to_string();
    } else if OS == "macos" {
        return find_macos_bin("mpv".to_string());
    }
    return find_executable_path_windows("mpv.exe");
}

fn find_executable_path_windows(executable: &str) -> String {
    let output = std::process::Command::new("where.exe")
        .arg(executable)
        .output()
        .expect("Failed to execute where.exe");

    if !output.status.success() {
        eprintln!("Failed to find mpv executable");
        return "mpv".to_string();
    }

    let path = String::from_utf8_lossy(&output.stdout);
    let path = path.trim();
    if path.is_empty() {
        eprintln!("Could not find mpv binary in common paths, falling back to bundled binary");
        return "mpv".to_string();
    }
    return path.to_string();
}

const MACOS_POTENTIAL_PATHS: [&str; 3] = [
    "/opt/local/bin",    // MacPorts
    "/opt/homebrew/bin", // Homebrew on AARCH64 Mac
    "/usr/local/bin",    // Homebrew on AMD64 Mac
];

fn find_macos_bin(bin: String) -> String {
    return MACOS_POTENTIAL_PATHS
        .iter()
        .map(|path| {
            let mut path = Path::new(path).to_path_buf();
            path.push(&bin);
            return path;
        })
        .find(|path| path.exists())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            println!("Could not find mpv binary in common paths, falling back to bundled binary");
            return bin;
        });
}
