use super::utils::{get_current, get_missing, mk_new_grade};
use super::{DEFAULT_DECENT, Grade, MIN_TOTAL, PanicCli};

pub fn calc_minimum(p: &PanicCli) -> Grade {
    let current = get_current(&p);
    let missing = get_missing(&p, &current, &MIN_TOTAL);

    mk_new_grade(p, &missing)
}

pub fn calc_decent(p: &PanicCli) -> Grade {
    let current = get_current(&p);
    let missing = get_missing(&p, &current, &p.decent.unwrap_or(DEFAULT_DECENT));

    mk_new_grade(&p, &missing)
}
