use std::fs;
use std::process::Command;

pub fn get_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap()
        .trim()
        .to_string()
}

pub fn get_kernel() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .unwrap();

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_ram() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();

    for line in meminfo.lines() {
        if line.starts_with("MemTotal") {
            return String::from_utf8_lossy(line.trim().as_bytes()).to_string();
        }
    }
    String::from("Unknown RAM")
}