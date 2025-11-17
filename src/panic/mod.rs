mod calc;
pub mod cli;
mod data;
mod utils;

use calc::calc_target;
use cli::PanicCli;
use data::Input;
use utils::print_result;

pub fn run(args: PanicCli) -> () {
    let input = Input {
        first: args.first,
        second: args.second,
    };

    let need = calc_target(&input, &args.minimum);
    let want = calc_target(&input, &args.target);

    print_result("Need", &need);
    print_result("Want", &want);
}
