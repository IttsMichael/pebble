use crossterm::style::Stylize;
use crate::models::package::data::Package;

pub fn process(results: &Vec<Package>) {
    println!("{}", "\nSearching...\n".cyan().bold());
    
    if results.is_empty() {
        println!("{}", "No packages found.".red());
        return;
    }

    for (i, pkg) in results.iter().enumerate() {
        let index_str = format!("[{}]", i + 1).yellow().bold();
        println!("{} {}", index_str, pkg.name.clone().green().bold());
        println!("    {}", pkg.description.clone().dim());
    }
}
