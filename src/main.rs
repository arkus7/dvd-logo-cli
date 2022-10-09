use clap::Parser;
use dvd_logo::{random_color, Cli};
use owo_colors::OwoColorize;

fn main() {
    let _cli = Cli::parse();

    let color = random_color();
    let debug_strings = [
        "         ",
        "         ",
        "   DVD   ",
        "  VIDEO  ",
        "         ",
        "         ",
    ];
    debug_strings
        .iter()
        .for_each(|s| println!("{}", s.color(color)));
}
