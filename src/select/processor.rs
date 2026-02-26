use crate::models::package::data::Package;
use crate::select::{prompt, confirm};

pub fn process(results: Vec<Package>) -> Option<Package> {
    if results.is_empty() { return None; }
    
    let index = prompt::processor::process(results.len());
    let selected = results.get(index - 1).cloned();
    
    if let Some(pkg) = &selected {
        if confirm::processor::process(pkg) {
            return selected;
        }
    }
    
    None
}
