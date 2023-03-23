use std::fs;

struct TextFile {
    path: String,
}

impl TextFile {
    fn get_contents(&self) -> String {
        fs::read_to_string(&self.path.trim()).expect("There was an error reading the file.")
    }

    fn get_total_whitespaces(&self) -> usize {
        let contents = self.get_contents();
        contents.chars().filter(|char| char.is_whitespace()).count()
    }

    fn get_total_alphabetic(&self) -> usize {
        let contents = self.get_contents();
        contents.chars().filter(|char| char.is_alphabetic()).count()
    }

    fn get_total_numbers(&self) -> usize {
        let contents = self.get_contents();
        contents.chars().filter(|char| char.is_numeric()).count()
    }

    fn get_total_characters(&self) -> usize {
        let contents = self.get_contents();
        contents.chars().count()
    }

    fn get_total_words(&self) -> usize {
        let contents = self.get_contents();
        contents.split_whitespace().count()
    }
}

pub fn get_report(path: String) {
    let text_file = TextFile { path };
    println!("This is the report for your file:");
    println!(
        "Alphabetic characters: {}",
        text_file.get_total_alphabetic()
    );
    println!("Numeric characters: {}", text_file.get_total_numbers());
    println!("Whitespaces: {}", text_file.get_total_whitespaces());
    println!("Total characters: {}", text_file.get_total_characters());
    println!("Total words: {}", text_file.get_total_words());
}
