use crate::search::{execute, format};
use crate::models::package::data::Package;

pub fn process(flag: Option<String>) -> Vec<Package> {
    if let Some(f) = flag {
        let results = execute::processor::process(&f);
        format::processor::process(&results);
        return results;
    }
    Vec::new()
}
