use clap::Parser;

use panic_man::panic::PanicCli;
use panic_man::panic::calc::{calc_decent, calc_minimum};
use panic_man::panic::utils::get_final;

fn main() {
    let args = PanicCli::parse();

    let need = calc_minimum(&args);
    let want = calc_decent(&args);

    println!("{:#?}", need);
    println!("{:?}", get_final(need));

    println!("{:#?}", want);
    println!("{:?}", get_final(want));
}
