use super::{Grade, PanicCli};

pub fn get_final(g: Grade) -> f32 {
    (g.first + g.second + g.third) / 3.0
}

pub fn get_current(p: &PanicCli) -> f32 {
    if let Some(x) = p.second {
        (p.first + x) / 3.0
    } else {
        p.first / 3.0
    }
}

pub fn get_missing(p: &PanicCli, current: &f32, minimum: &f32) -> f32 {
    if let Some(_) = p.second {
        (minimum - current) * 3.0
    } else {
        (minimum - current) * 3.0 * 0.5
    }
}

pub fn mk_new_grade(p: &PanicCli, missing: &f32) -> Grade {
    if let Some(x) = p.second {
        Grade {
            first: p.first,
            second: x,
            third: *missing,
        }
    } else {
        Grade {
            first: p.first,
            second: *missing,
            third: *missing,
        }
    }
}
