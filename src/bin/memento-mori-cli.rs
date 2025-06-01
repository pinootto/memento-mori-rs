use clap::Parser;
use memento_mori_rs::launch;
use memento_mori_rs::Args;

fn main() {
    let args = Args::parse();
    let output = launch(args);
    println!("{}", output);
}
