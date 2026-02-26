mod display;
mod input;
mod search;
mod models;
mod select;
mod install;

use crate::models::AppConfig;

fn main() {
    let config = AppConfig::parse_args();

    display::process();
    
    let search_query = input::process();
    
    let search_results = search::process(Some(search_query));
    
    let selected_package = select::process(search_results);
    
    install::process(selected_package, config);
}

// test