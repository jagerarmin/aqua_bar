use chrono::Local;

pub fn get_time(format: &str) -> String {
    let now = Local::now();
    now.format(format).to_string()
}
