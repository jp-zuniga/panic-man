#[derive(Debug)]
pub(crate) struct Grade {
    pub(crate) first: f64,
    pub(crate) second: f64,
    pub(crate) third: f64,
}

#[derive(Debug)]
pub(crate) struct Input {
    pub(crate) first: f64,
    pub(crate) second: Option<f64>,
}
