pub struct Cipher {
    text: String,
    shift: u64,
    text_encrypted: String,
}

impl Cipher {
    pub fn new() -> Cipher {
        Cipher {
            text: String::new(),
            shift: 0,
            text_encrypted: String::new(),
        }
    }
    pub fn add_string(&mut self, text: &str) {
        if text.is_ascii() {
            self.text.push_str(text.to_lowercase().as_str());
        }
    }

    pub fn set_shift(&mut self, shift: u64) {
        self.shift = shift;
    }

    pub fn encrypt(&mut self) -> String{

        let ascii_alphabet = "abcdefghijklmnopqrstuvwxyz";

        let text_parts: Vec<char> = self.text.chars().collect();

        for part in text_parts {
            let position = ascii_alphabet.find(part).unwrap() as u64;

            let mut shifted_position = position + self.shift;

            if shifted_position >= 26 {
                shifted_position = shifted_position -26;
            }

            let shifted_char: String = ascii_alphabet.chars().skip(shifted_position as usize).take(1).collect();
            self.text_encrypted.push_str(&shifted_char)
        }

        let returnable = &self.text_encrypted;
        return returnable.to_string();
    }
}