use crate::input::{prompt, read};

pub fn process() -> String {
    prompt::processor::process();
    read::processor::process()
}
