use super::data::{Grade, Input};

pub(crate) const MIN_TOTAL: f64 = 70.0;
pub(crate) const DEFAULT_TARGET: f64 = 85.0;

pub(crate) fn get_final(g: &Grade) -> f64 {
    (g.first + g.second + g.third) / 3.0
}

pub(crate) fn get_current(i: &Input) -> f64 {
    if let Some(x) = i.second {
        (i.first + x) / 3.0
    } else {
        i.first / 3.0
    }
}

pub(crate) fn get_missing(i: &Input, current: &f64, target: &f64) -> f64 {
    if let Some(_) = i.second {
        (target - current) * 3.0
    } else {
        (target - current) * 3.0 * 0.5
    }
}

pub(crate) fn mk_new_grade(i: &Input, missing: &f64) -> Grade {
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

pub(crate) fn parse_grade(s: &str) -> Result<f64, String> {
    let value = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number."))?;

    if (0.0..=100.0).contains(&value) {
        Ok(value)
    } else {
        Err(format!("grade must be between 0 and 100, got {value}."))
    }
}

pub(crate) fn print_result(header: &str, grade: &Grade) {
    let avg = get_final(grade);

    println!("{header}:");
    println!("  Final average: {:.2}", avg);
    println!("  Grade breakdown:");
    println!("    C1: {:.2}", grade.first);
    println!("    C2: {:.2}", grade.second);
    println!("    C3: {:.2}", grade.third);
    println!();
}
