use super::data::{Grade, Input};
use super::utils::{get_current, get_missing, mk_new_grade};

pub fn calc_target(i: &Input, target: &f64) -> Grade {
    let current = get_current(&i);
    let missing = get_missing(&i, &current, &target);

    mk_new_grade(&i, &missing)
}
