use std::io;

pub fn process() -> String {
    let mut searchv = String::new();
    io::stdin()
        .read_line(&mut searchv)
        .expect("Failed to read line");
    
    searchv.trim().to_string()
}
