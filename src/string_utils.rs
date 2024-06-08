pub fn is_alphabetic(txt: &String) -> bool {
    txt.to_lowercase() != txt.to_uppercase()
}

pub fn is_numeric(txt: &String) -> bool {
    txt.parse::<u8>().is_ok()
}

pub fn char_at(txt: &String, pos: usize) -> char {
    let chars: Vec<char> = txt.chars().collect();

    chars[pos]
}
