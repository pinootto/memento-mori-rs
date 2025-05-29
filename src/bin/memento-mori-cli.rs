use clap::Parser;
use memento_mori_rs::launch;
use memento_mori_rs::Cli;

fn main() {
    let cli = Cli::parse();
    launch(cli);
}
