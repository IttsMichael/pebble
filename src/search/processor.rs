use crate::search::{execute, format};

pub fn process(flag: Option<String>) -> Option<String> {
    if let Some(f) = flag {
        let result = execute::processor::process(&f);
        format::processor::process(result);
    }
    None 
}
