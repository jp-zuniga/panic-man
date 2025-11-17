use clap::Parser;

use panic_man::panic::calc::calc_target;
use panic_man::panic::cli::PanicCli;
use panic_man::panic::data::Input;
use panic_man::panic::utils::get_final;

fn main() {
    let args = PanicCli::parse();

    let input = Input {
        first: args.first,
        second: args.second,
    };

    let need = calc_target(&input, &args.minimum);
    let want = calc_target(&input, &args.target);

    println!("{:#?}", need);
    println!("{:?}", get_final(&need));

    println!("{:#?}", want);
    println!("{:?}", get_final(&want));
}
