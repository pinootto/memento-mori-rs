use clap::Parser;
use memento_mori_rs::build_calendar;
use memento_mori_rs::Args;

fn main() {
    let args = Args::parse();
    let output = build_calendar(args);
    println!("{}", output);
}
