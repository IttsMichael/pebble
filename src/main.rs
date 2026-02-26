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
    
    loop {
        let search_query = input::process();
        let search_results = search::process(Some(search_query));
        
        let selected_package = select::process(search_results);
        
        // If they pick a package, move to install and break out.
        // If they input 0 to cancel, it falls through to continue the loop.
        if let Some(pkg) = selected_package {
            install::process(Some(pkg), config.clone());
            break;
        } else {
            // Cancelled out of select early, clear and query again
            display::process();
        }
    }
}