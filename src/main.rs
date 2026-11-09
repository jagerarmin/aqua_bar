mod battery;
mod time;

fn main() {
    let cap = battery::read_capacity().unwrap();
    let stat = battery::read_status().unwrap();
    let charging = stat == "Charging" || stat == "Full";
    println!(
        "Charge:{}% Charging:{}",
        cap,
        if charging { "Yes" } else { "No" }
    );

    let user_format = "%Y:%H:%M";
    let clock = time::get_time(user_format);
    println!("Time {}", clock);
}
