use std::io;

mod utils;

fn main() {
    let mut raw_path = String::new();

    println!("Type in the absolute or relative path for the file:");

    io::stdin()
        .read_line(&mut raw_path)
        .expect("File path couldn't be found.");

    utils::get_report(raw_path);
}
