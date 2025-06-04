use rand::distr::{Alphanumeric, SampleString};

pub fn username() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 16)
}
