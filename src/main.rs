mod display;
mod input;
mod search;

fn main() {
    display::processor::process();
    
    let searchv = input::processor::process();
    
    search::processor::process(Some(searchv));
}