use chrono::{Datelike, Local, NaiveDate, Timelike};
use std::env;
use webbrowser;

const LUNCH_URL: &str = "https://apps.jesus.cam.ac.uk/foodmenuview/digiboard.php?event_id=1";
const DINNER_URL: &str = "https://apps.jesus.cam.ac.uk/foodmenuview/digiboard.php?event_id=2";
const FORMAL_BASE_URL: &str = "https://apps.jesus.cam.ac.uk/foodmenuview/digiboard.php?event_id=5";

fn open_url(url: &str) {
    if let Err(e) = webbrowser::open(url) {
        eprintln!("Failed to open browser: {}", e);
    }
}

fn parse_date(date_str: &str) -> Option<NaiveDate> {
    let now = Local::now();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();

    match date_str.len() {
        // DD format
        2 => {
            let day: u32 = date_str.parse().ok()?;
            let mut year = current_year;
            let mut month = current_month;

            // If day is past current day, increment month
            if day < current_day {
                if month == 12 {
                    year += 1;
                    month = 1;
                } else {
                    month += 1;
                }
            }

            NaiveDate::from_ymd_opt(year, month, day)
        }
        // MM-DD format
        5 => {
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() != 2 {
                return None;
            }
            let month: u32 = parts[0].parse().ok()?;
            let day: u32 = parts[1].parse().ok()?;
            NaiveDate::from_ymd_opt(current_year, month, day)
        }
        // YYYY-MM-DD format
        10 => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok(),
        _ => None,
    }
}

fn is_formal_day(date: NaiveDate) -> bool {
    // 1 = Monday, ..., 7 = Sunday
    matches!(date.weekday().number_from_monday(), 2 | 3 | 5 | 7)
}

fn print_help() {
    println!(
        r#"Script to open Jesus College Caff Menu.
Running with arguments will result in opening Lunch Menu if before 1pm, and Dinner Menu if after 1pm.
Running with:
    l - Lunch Menu
    d - Dinner Menu
    f - Formal Menu
    f DD - Formal Menu of Day
    f MM-DD - Formal Menu of Month and Day"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str);

    match command {
        Some("l") => open_url(LUNCH_URL),
        Some("d") => open_url(DINNER_URL),
        Some("f") => {
            match args.get(2) {
                None => open_url(FORMAL_BASE_URL),
                Some(date_str) => {
                    if let Some(date) = parse_date(date_str) {
                        if is_formal_day(date) {
                            let formal_url = format!("{}&date={}", FORMAL_BASE_URL, date);
                            open_url(&formal_url);
                        } else {
                            println!("There is no formal hall on this day. Formal halls are only on Tuesdays, Wednesdays, Fridays, and Sundays.");
                        }
                    } else {
                        println!("Invalid date format. Please use DD, MM-DD, or YYYY-MM-DD");
                    }
                }
            }
        }
        Some("help" | "h") => print_help(),
        _ => {
            // Default behavior based on time of day
            let hour = Local::now().hour();
            if hour > 13 {
                open_url(DINNER_URL);
            } else {
                open_url(LUNCH_URL);
            }
        }
    }
}
