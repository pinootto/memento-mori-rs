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

    let mut output = String::new();

    build_intro(&mut output);

    let death_date = cli.birthday + (cli.death_age as i32).years();
    output.push_str(
        format!(
            "if you live {} years, your death day will be {}\n",
            cli.death_age, death_date
        )
        .as_str(),
    );

    let birthday_timestamp = cli.birthday.in_tz("America/New_York").unwrap().timestamp();

    let death_timestamp = death_date.in_tz("America/New_York").unwrap().timestamp();

    let now = Timestamp::now();

    let current_elapsed = now - birthday_timestamp;
    // println!("current elapsed = {}", current_elapsed);

    let life_elapsed = death_timestamp - birthday_timestamp;

    match cli.time_unit {
        TimeUnit::Week => build_output_by_week(&cli, &current_elapsed, &life_elapsed, &mut output),
        TimeUnit::Month => {
            build_output_by_month(&cli, &current_elapsed, &life_elapsed, &mut output)
        }
    };

    println!("{}", output);
}

fn build_output_by_week(
    cli: &Cli,
    current_elapsed: &Span,
    life_elapsed: &Span,
    output: &mut String,
) {
    let current_week = current_elapsed
        .total(SpanTotal::from(Unit::Week).days_are_24_hours())
        .unwrap();

    let life_weeks = life_elapsed
        .total(SpanTotal::from(Unit::Week).days_are_24_hours())
        .unwrap();

    output.push_str(
        format!(
            "already passed {} weeks in your life, out of {} weeks ({} years)\n",
            current_week as u16, life_weeks as u16, cli.death_age
        )
        .as_str(),
    );

    output.push_str(
        format!(
            "{}% of your life is passed\n
            don't waste your remaining time\n",
            (current_week / life_weeks * 100.0) as u8
        )
        .as_str(),
    );

    let mut week_counter = 0.0;
    let week_scaler = WEEKS_IN_A_YEAR / 52.0;
    // println!("week_scaler = {}", week_scaler);
    output.push('\n');
    output.push_str("year weeks\n");
    for year in 0..cli.death_age {
        output.push_str(format!("{:0>3}  ", year).as_str());
        for _week in 0..52 {
            if week_counter < current_week {
                output.push('#');
            } else {
                output.push('.');
            }
            week_counter += week_scaler;
        }
        output.push('\n');
    }
    output.push_str(format!("{:0>3}  ", cli.death_age).as_str());
}

fn build_output_by_month(
    cli: &Cli,
    current_elapsed: &Span,
    life_elapsed: &Span,
    output: &mut String,
) {
    let current_month = current_elapsed.total((Unit::Month, cli.birthday)).unwrap();

    let life_months = life_elapsed.total((Unit::Month, cli.birthday)).unwrap();

    output.push_str(
        format!(
            "already passed {} months in your life, out of {} months ({} years)\n",
            current_month as u16, life_months as u16, cli.death_age
        )
        .as_str(),
    );

    output.push_str(
        format!(
            "{}% of your life is passed\n
            don't waste your remaining time\n",
            (current_month / life_months * 100.0) as u8
        )
        .as_str(),
    );

    let mut month_counter = 0.0;
    output.push('\n');
    output.push_str("year months\n");
    for year in 0..cli.death_age {
        output.push_str(format!("{:0>3}  ", year).as_str());
        for _month in 0..12 {
            if month_counter < current_month {
                output.push('#');
            } else {
                output.push('.');
            }
            month_counter += 1.0;
        }
        output.push('\n');
    }
    output.push_str(format!("{:0>3}  ", cli.death_age).as_str());
}

fn build_intro(output: &mut String) {
    output.push_str("------------\n");
    output.push_str("memento mori - remember that you will die\n");
    output.push('\n');
}
