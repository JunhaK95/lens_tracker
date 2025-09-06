use chrono::{Datelike, Local, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write, stdin};

fn main() {
    println!("Would you like to update or create a new lense?");

    let mut choice = String::new();

    let filepath = "lens_data.json";
    stdin().read_line(&mut choice).expect("Please give string");

    choice = choice.trim().to_lowercase();

    if choice == "new" {
        let lens = build_lens();

        save_lens(&lens, filepath);
        println!(
            "New lens created!\nOpened: {},\nDays used: {}\nDays left: {}",
            lens.opened_date,
            lens.days_used,
            30 - lens.days_used
        )
    } else if choice == "update" {
        let mut lens = load_lens(filepath);

        let today = get_date();

        if lens.last_updated != today.date {
            lens.last_updated = today.date;
            lens.days_used += 1;

            println!(
                "Lens updated successfully!\nOpened: {},\nDays used: {},\nDays left: {}",
                lens.opened_date,
                lens.days_used,
                30 - lens.days_used
            );
            save_lens(&lens, filepath)
        } else {
            println!(
                "Lens already updated today!\nOpened: {},\nDays used: {},\nDays left: {}",
                lens.opened_date,
                lens.days_used,
                30 - lens.days_used
            );
        }
    }
}

struct Date {
    day_of_week: Weekday,
    date: NaiveDate,
}

#[derive(Deserialize, Serialize)]
struct Lens {
    opened_date: NaiveDate,
    weekday: Weekday,
    last_updated: NaiveDate,
    days_used: u32,
}

fn build_lens() -> Lens {
    let date = get_date();

    Lens {
        opened_date: date.date,
        weekday: date.day_of_week,
        last_updated: date.date,
        days_used: 1,
    }
}

fn get_date() -> Date {
    let now = Local::now();

    Date {
        date: now.date_naive(),
        day_of_week: now.weekday(),
    }
}

fn save_lens(lens: &Lens, filename: &str) {
    let json = serde_json::to_string_pretty(lens).expect("Failed to serialize lens");

    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write data");
}

fn load_lens(filename: &str) -> Lens {
    let file = File::open(filename).expect("Failed to open lens data file");
    serde_json::from_reader(file).expect("Failed to deserialize lens data")
}
