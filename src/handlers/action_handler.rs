use std::process::Command;

#[cfg(target_os = "linux")]
pub fn lock_device() {
    Command::new("loginctl").args(["lock-session"]).spawn().unwrap();
}

#[cfg(target_os = "windows")]
pub fn lock_device() {
    Command::new("Rundll32.exe").args(["user32.dll,LockWorkStation"]).spawn().unwrap();
}

#[cfg(target_os = "macos")]
pub fn lock_device() {
    // NEVER TESTED BEFORE
    Command::new("pmset").args(["displaysleepnow"]).spawn().unwrap();
}