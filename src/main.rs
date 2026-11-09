mod battery;
mod time;

fn main() {
    loop {
        let cap = battery::read_capacity().unwrap_or(0);
        let stat = battery::read_status().unwrap_or_else(|| "Unknown".to_string());
        let charging = stat == "Charging" || stat == "Full";
        println!(
            "Charge:{}% Charging:{}",
            cap,
            if charging { "Yes" } else { "No" }
        );

        let user_format = "%Y:%H:%M:%S";
        let clock = time::get_time(user_format);
        println!("Time {}", clock);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
