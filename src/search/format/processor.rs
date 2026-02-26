use crossterm::style::Stylize;

pub fn process(result: Option<String>) {
    if let Some(content) = result {
        println!("{}", "\nSearching...".cyan().bold());
        println!("{}", content.green());
    }
}
