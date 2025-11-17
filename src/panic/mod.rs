use clap::Parser;

pub mod calc;
pub mod utils;

pub const MIN_TOTAL: f32 = 70.0;
pub const DEFAULT_DECENT: f32 = 85.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PanicCli {
    #[arg(short = 'f', long = "first")]
    first: f32,

    #[arg(short = 's', long = "second")]
    second: Option<f32>,

    #[arg(short = 'd', long = "decent")]
    decent: Option<f32>,
}

#[derive(Debug)]
pub struct Grade {
    first: f32,
    second: f32,
    third: f32,
}
