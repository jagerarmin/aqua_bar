use std::fs;
use std::path::PathBuf;
//use std::io;
//use std::path;

pub fn find_buttery() -> Option<PathBuf> {
    let dir = fs::read_dir("/sys/class/power_supply").ok()?;
    for entry in dir.flatten() {
        let path = entry.path();
        let type_file = path.join("type");
        if let Ok(t) = fs::read_to_string(&type_file) {
            if t.trim() == "Battery" {
                return Some(path);
            }
        }
    }
    None
}

pub fn read_capacity() -> Option<u8> {
    let path = find_buttery()?;
    let text = fs::read_to_string(path.join("capacity")).ok()?;
    text.trim().parse::<u8>().ok()
}

pub fn read_status() -> Option<String> {
    let path = find_buttery()?;
    let text = fs::read_to_string(path.join("status")).ok()?;
    Some(text.trim().to_string())
}
