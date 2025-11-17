use super::data::{Grade, Input};

pub const MIN_TOTAL: f64 = 70.0;
pub const DEFAULT_TARGET: f64 = 85.0;

pub fn get_final(g: &Grade) -> f64 {
    (g.first + g.second + g.third) / 3.0
}

pub fn get_current(i: &Input) -> f64 {
    if let Some(x) = i.second {
        (i.first + x) / 3.0
    } else {
        i.first / 3.0
    }
}

pub fn get_missing(i: &Input, current: &f64, target: &f64) -> f64 {
    if let Some(_) = i.second {
        (target - current) * 3.0
    } else {
        (target - current) * 3.0 * 0.5
    }
}

pub fn mk_new_grade(i: &Input, missing: &f64) -> Grade {
    if let Some(x) = i.second {
        Grade {
            first: i.first,
            second: x,
            third: *missing,
        }
    } else {
        Grade {
            first: i.first,
            second: *missing,
            third: *missing,
        }
    }
}

pub fn parse_grade(s: &str) -> Result<f64, String> {
    let value = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number."))?;

    if (0.0..=100.0).contains(&value) {
        Ok(value)
    } else {
        Err(format!("grade must be between 0 and 100, got {value}."))
    }
}
