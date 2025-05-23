use std::fmt::Display;

use clap::{Parser, ValueEnum};
use jiff::civil::Date;
use jiff::{SpanTotal, Timestamp, ToSpan, Unit};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    birthday: Date,
    #[arg(default_value_t = 90)]
    #[arg(value_parser = clap::value_parser!(u8).range(0..=150))]
    death_age: u8,
    #[arg(short, long)]
    #[arg(default_value_t = TimeUnit::Month)]
    time_unit: TimeUnit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TimeUnit {
    Week,
    Month,
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
        }
    }
}

const WEEKS_IN_A_YEAR: f64 = 52.18; // roughly adjusted for leap years

pub fn launch() {
    let cli = Cli::parse();

    // println!("{:#?}", cli);

    let death_date = cli.birthday + (cli.death_age as i32).years();
    println!("your death day: {death_date}");

    let birthday_timestamp = cli.birthday.in_tz("America/New_York").unwrap().timestamp();

    let death_timestamp = death_date.in_tz("America/New_York").unwrap().timestamp();

    let now = Timestamp::now();

    let current_elapsed = now - birthday_timestamp;
    // println!("current elapsed = {}", current_elapsed);

    let current_week = current_elapsed
        .total(SpanTotal::from(Unit::Week).days_are_24_hours())
        .unwrap();

    let life_weeks = (death_timestamp - birthday_timestamp)
        .total(SpanTotal::from(Unit::Week).days_are_24_hours())
        .unwrap();

    println!(
        "current week in your life: {} out of {} weeks ({} years)",
        current_week as u16, life_weeks as u16, cli.death_age
    );

    println!(
        "{}% of your life is passed",
        (current_week / life_weeks * 100.0) as u8
    );

    let mut week_counter = 0.0;
    let week_scaler = WEEKS_IN_A_YEAR / 52.0;
    // println!("week_scaler = {}", week_scaler);
    println!();
    println!("year weeks");
    for year in 0..cli.death_age {
        print!("{:0>3}  ", year);
        for _week in 0..52 {
            if week_counter < current_week {
                print!("#");
            } else {
                print!(".");
            }
            week_counter += week_scaler;
        }
        println!();
    }
    println!("{:0>3}  ", cli.death_age);
}
