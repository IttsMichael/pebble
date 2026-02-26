use crate::models::package::data::Package;
use crate::models::config::data::AppConfig;
use crate::install::{execute, beta};
use crossterm::style::Stylize;

pub fn process(package: Option<Package>, config: AppConfig) {
    match package {
        Some(pkg) => {
            if config.beta_mode {
                beta::processor::process(&pkg); 
            } else {
                execute::processor::process(&pkg);
            }
        }
        None => {
            println!("{}", "aborted".red());
        }
    }
}
