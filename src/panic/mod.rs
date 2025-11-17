use clap::Parser;

pub mod calc;
pub mod utils;

pub const MIN_TOTAL: f64 = 70.0;
pub const DEFAULT_TARGET: f64 = 85.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PanicCli {
    #[arg(short = 'f', long = "first")]
    first: f64,

    #[arg(short = 's', long = "second")]
    second: Option<f64>,

    #[arg(short = 't', long = "target")]
    target: Option<f64>,
}

#[derive(Debug)]
pub struct Grade {
    first: f64,
    second: f64,
    third: f64,
}
