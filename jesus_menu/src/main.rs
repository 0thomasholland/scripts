use chrono::{Datelike, Local, NaiveDate, Timelike, Weekday};
use std::env;
use webbrowser;

const LUNCH_URL: &str = "https://apps.jesus.cam.ac.uk/foodmenuview/digiboard.php?event_id=1";
const DINNER_URL: &str = "https://apps.jesus.cam.ac.uk/foodmenuview/digiboard.php?event_id=2";
const FORMAL_BASE_URL: &str = "https://apps.jesus.cam.ac.uk/foodmenuview/digiboard.php?event_id=5";

#[derive(Debug)]
enum MenuError {
    InvalidDate(String),
    BrowserError(String),
}

impl std::fmt::Display for MenuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuError::InvalidDate(msg) => write!(f, "Invalid date: {}", msg),
            MenuError::BrowserError(msg) => write!(f, "Browser error: {}", msg),
        }
    }
}

impl std::error::Error for MenuError {}

fn open_url(url: &str) -> Result<(), MenuError> {
    webbrowser::open(url).map_err(|e| MenuError::BrowserError(e.to_string()))
}

fn parse_date(date_str: &str) -> Result<NaiveDate, MenuError> {
    let now = Local::now();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();

    match date_str.len() {
        // DD format
        2 => {
            let day: u32 = date_str.parse().map_err(|_| MenuError::InvalidDate("Invalid day format".to_string()))?;
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
                .ok_or_else(|| MenuError::InvalidDate("Invalid date combination".to_string()))
        }
        // MM-DD format
        5 => {
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() != 2 {
                return Err(MenuError::InvalidDate("Invalid MM-DD format".to_string()));
            }
            let month: u32 = parts[0].parse()
                .map_err(|_| MenuError::InvalidDate("Invalid month".to_string()))?;
            let day: u32 = parts[1].parse()
                .map_err(|_| MenuError::InvalidDate("Invalid day".to_string()))?;
            
            NaiveDate::from_ymd_opt(current_year, month, day)
                .ok_or_else(|| MenuError::InvalidDate("Invalid date combination".to_string()))
        }
        // YYYY-MM-DD format
        10 => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| MenuError::InvalidDate("Invalid YYYY-MM-DD format".to_string())),
        _ => Err(MenuError::InvalidDate("Invalid date format. Use DD, MM-DD, or YYYY-MM-DD".to_string()))
    }
}

fn is_formal_day(date: NaiveDate) -> bool {
    use Weekday::*;
    matches!(date.weekday(), Tue | Wed | Fri | Sun)
}

fn print_help() {
    println!(
        r#"Jesus College Menu Viewer

USAGE:
    jesus_menu [COMMAND]

COMMANDS:
    l           Open lunch menu
    d           Open dinner menu
    f           Open formal menu
    f DD        Open formal menu for specific day
    f MM-DD     Open formal menu for specific month and day
    f YYYY-MM-DD Open formal menu for specific date
    h, help     Print this help message

If no command is provided, opens lunch menu before 1pm and dinner menu after."#
    );
}

fn run() -> Result<(), MenuError> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str);

    match command {
        Some("l") => open_url(LUNCH_URL),
        Some("d") => open_url(DINNER_URL),
        Some("f") => {
            match args.get(2) {
                None => open_url(FORMAL_BASE_URL),
                Some(date_str) => {
                    let date = parse_date(date_str)?;
                    if is_formal_day(date) {
                        let formal_url = format!("{}&date={}", FORMAL_BASE_URL, date);
                        open_url(&formal_url)
                    } else {
                        println!("There is no formal hall on this day.");
                        println!("Formal halls are only on Tuesdays, Wednesdays, Fridays, and Sundays.");
                        Ok(())
                    }
                }
            }
        }
        Some("help" | "h") => {
            print_help();
            Ok(())
        }
        _ => {
            let hour = Local::now().hour();
            if hour > 13 {
                open_url(DINNER_URL)
            } else {
                open_url(LUNCH_URL)
            }
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
