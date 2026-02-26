use crate::models::package::data::Package;
use crate::install::execute;
use crossterm::style::Stylize;

pub fn process(package: Option<Package>) {
    match package {
        Some(pkg) => {
            execute::processor::process(&pkg);
        }
        None => {
            println!("{}", "aborted".red());
        }
    }
}
