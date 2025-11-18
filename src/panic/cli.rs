use clap::Parser;
use clap_builder::{crate_authors, crate_version};

use super::utils::{DEFAULT_TARGET, MIN_TOTAL, parse_grade};

#[derive(Parser, Debug)]
#[command(author = crate_authors!(), version = crate_version!(), about = None, long_about = None)]
pub struct PanicCli {
    /// Nota mínima para pasar el curso (0-100).
    #[arg(short, long, default_value_t = MIN_TOTAL, value_parser = parse_grade)]
    pub(crate) minimum: f64,

    /// Nota final deseada (0-100).
    #[arg(short, long, default_value_t = DEFAULT_TARGET, value_parser = parse_grade)]
    pub(crate) target: f64,

    /// Nota del primer corte (0-100).
    #[arg(short, long, value_parser = parse_grade)]
    pub(crate) first: f64,

    /// Nota del segundo corte (0-100). Opcional.
    #[arg(short, long, value_parser = parse_grade)]
    pub(crate) second: Option<f64>,
}
