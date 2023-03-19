use std::fs;

fn get_whitespaces_total(data: &String) -> usize {
    data.chars().filter(|char| char.is_whitespace()).count()
}

fn get_alphabetic_total(data: &String) -> usize {
    data.chars().filter(|char| char.is_alphabetic()).count()
}

fn get_numeric_total(data: &String) -> usize {
    data.chars().filter(|char| char.is_numeric()).count()
}

fn get_characters_total(data: &String) -> usize {
    data.chars().count()
}

fn get_words_total(data: &String) -> usize {
    data.split_whitespace().count()
}

fn get_file_contents(path: String) -> String {
    fs::read_to_string(path.trim()).expect("File contents couldn't be loaded.")
}

pub fn get_report(path: String) {
    let contents = get_file_contents(path);
    println!("This is the report for your file:");
    println!("Alphabetic characters: {}", get_alphabetic_total(&contents));
    println!("Numeric characters: {}", get_numeric_total(&contents));
    println!("Whitespaces: {}", get_whitespaces_total(&contents));
    println!("Total characters: {}", get_characters_total(&contents));
    println!("Total words: {}", get_words_total(&contents));
}
