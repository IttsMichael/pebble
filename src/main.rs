mod display;
mod input;
mod search;
mod models;
mod select;
mod install;

use crate::models::config::data::AppConfig;

fn main() {
    let config = AppConfig::parse_args();

    display::processor::process();
    
    let search_query = input::processor::process();
    
    let search_results = search::processor::process(Some(search_query));
    
    let selected_package = select::processor::process(search_results);
    
    install::processor::process(selected_package, config);
}