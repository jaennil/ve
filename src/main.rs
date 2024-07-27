use std::{io::Write as _, process::Command};

use arboard::Clipboard;
use enigo::{Enigo, Keyboard as _, Settings};

fn main() -> std::io::Result<()> {
    let tmp_path = std::path::Path::new("/tmp/ve/data");

    let mut clipboard = Clipboard::new().unwrap();
    let clipboard_text = clipboard.get_text().unwrap();
    dbg!(&clipboard_text);

    let mut file: std::fs::File = std::fs::File::create(tmp_path)?;
    file.write_all(clipboard_text.as_bytes())?;

    let status = Command::new("alacritty")
        .arg("-e")
        .arg("nvim")
        .arg(tmp_path)
        .status()
        .expect("failed to execute process");
    dbg!(status);

    let contents = std::fs::read_to_string(tmp_path)?;
    dbg!(&contents);

    std::thread::sleep(std::time::Duration::from_secs(1));

    Enigo::new(&Settings::default()).unwrap().text(&contents).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}
