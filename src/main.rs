mod cipher;
use cipher::Cipher;
use console::Term;

fn main() {

    let stdout = Term::buffered_stdout();

    println!("Select example to run:");
    println!("1. Caesar Cipher");

    if let Ok(character) = stdout.read_char() {
        match character {
            '1' => caesar_cipher(),
            _ => println!("Nothing selected"),
        }
    }

}

fn caesar_cipher() {
    let mut cipher = Cipher::new();
    let text = "texthere";
    println!("Text in: {}", text);
    cipher.add_string(text);
    let shift:u64 = 1;
    cipher.set_shift(shift);
    println!("Text out: {}", cipher.encrypt());
}
