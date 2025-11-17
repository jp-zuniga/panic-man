use clap::Parser;
use panic_man::{PanicCli, run};

fn main() {
    run(PanicCli::parse());
}
