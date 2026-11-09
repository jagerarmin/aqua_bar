mod battery;

fn main() {
    let cap = battery::read_capacity().unwrap();
    let stat = battery::read_status().unwrap();
    let charging = stat == "Charging" || stat == "Full";
    println!(
        "Charge:{}% Charging:{}",
        cap,
        if charging { "Yes" } else { "No" }
    );
}
