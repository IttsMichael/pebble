use crate::display::{clear, title};

pub fn process() {
    clear::processor::process();
    title::processor::process();
}
