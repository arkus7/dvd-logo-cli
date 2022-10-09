use clap::Parser;
use owo_colors::Rgb;
use rand::Rng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {}

pub fn random_color() -> Rgb {
    let mut rng = rand::thread_rng();

    Rgb(
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    )
}
