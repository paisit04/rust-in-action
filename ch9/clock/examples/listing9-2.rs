use chrono::{Days, Duration, Local};

fn main() {
    let now = Local::now();
    println!("now: {}", now);

    let one_week_from_now = now.checked_add_days(Days::new(7)).unwrap();
    println!("one_week_from_now: {}", one_week_from_now);

    let two_weeks_from_now = now.checked_add_signed(Duration::weeks(2)).unwrap();
    println!("two_weeks_from_now: {}", two_weeks_from_now);
}
