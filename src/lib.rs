use clap::Parser;
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
}

pub fn launch() {
    let cli = Cli::parse();

    println!("{:#?}", cli);

    let death_date = cli.birthday + (cli.death_age as i32).years();
    println!("Your death day is {death_date}");

    let birthday_timestamp = cli.birthday.in_tz("America/New_York").unwrap().timestamp();

    let now = Timestamp::now();

    let current_elapsed = now - birthday_timestamp;
    println!("current elapsed = {}", current_elapsed);
    let current_week = current_elapsed
        .total(SpanTotal::from(Unit::Week).days_are_24_hours())
        .unwrap() as u32;
    println!("current week in your life: {}", current_week);

    let mut week_counter = 0;
    for year in 0..cli.death_age {
        print!("{}  ", year);
        for week in 0..52 {
            if week_counter < current_week {
                print!("#");
            } else {
                print!(".");
            }
            week_counter += 1;
        }
        println!();
    }
}
