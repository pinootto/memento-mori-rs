use std::fmt::Display;

use clap::{Parser, ValueEnum};
use jiff::civil::Date;
use jiff::{Span, SpanTotal, Timestamp, ToSpan, Unit};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    birthday: Date,
    #[arg(short, long)]
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
    println!("------------");
    println!("memento mori - remember that you will die");
    println!();

    let death_date = cli.birthday + (cli.death_age as i32).years();
    println!(
        "if you live {} years, your death day will be {}",
        cli.death_age, death_date
    );

    let birthday_timestamp = cli.birthday.in_tz("America/New_York").unwrap().timestamp();

    let death_timestamp = death_date.in_tz("America/New_York").unwrap().timestamp();

    let now = Timestamp::now();

    let current_elapsed = now - birthday_timestamp;
    // println!("current elapsed = {}", current_elapsed);

    let life_elapsed = death_timestamp - birthday_timestamp;

    match cli.time_unit {
        TimeUnit::Week => print_by_week(&cli, &current_elapsed, &life_elapsed),
        TimeUnit::Month => print_by_month(&cli, &current_elapsed, &life_elapsed),
    }
}

fn print_by_week(cli: &Cli, current_elapsed: &Span, life_elapsed: &Span) {
    let current_week = current_elapsed
        .total(SpanTotal::from(Unit::Week).days_are_24_hours())
        .unwrap();

    let life_weeks = life_elapsed
        .total(SpanTotal::from(Unit::Week).days_are_24_hours())
        .unwrap();

    println!(
        "already passed weeks in your life: {} out of {} weeks ({} years)",
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

fn print_by_month(cli: &Cli, current_elapsed: &Span, life_elapsed: &Span) {
    let current_month = current_elapsed.total((Unit::Month, cli.birthday)).unwrap();

    let life_months = life_elapsed.total((Unit::Month, cli.birthday)).unwrap();

    println!(
        "already passed months in your life: {} out of {} months ({} years)",
        current_month as u16, life_months as u16, cli.death_age
    );

    println!(
        "{}% of your life is passed",
        (current_month / life_months * 100.0) as u8
    );

    let mut month_counter = 0.0;
    println!();
    println!("year months");
    for year in 0..cli.death_age {
        print!("{:0>3}  ", year);
        for _month in 0..12 {
            if month_counter < current_month {
                print!("#");
            } else {
                print!(".");
            }
            month_counter += 1.0;
        }
        println!();
    }
    println!("{:0>3}  ", cli.death_age);
}
