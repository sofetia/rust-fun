slint::include_modules!();
mod cipher;
use cipher::Cipher;
use console::Term;

fn main() {

    let stdout = Term::buffered_stdout();

    println!("Select example to run:");
    println!("1. Caesar Cipher");
    println!("2. slint window");

    if let Ok(character) = stdout.read_char() {
        match character {
            '1' => caesar_cipher(),
            '2' => run_slint().expect("REASON"),
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

pub fn run_slint() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}
