use crate::handlers::keyboard_handler;
use enigo::Key;

pub async fn handle_presentation(msg: &str) {
    match msg.trim() {
        "left" => keyboard_handler::click(Key::LeftArrow).await,
        "right" => keyboard_handler::click(Key::RightArrow).await,
        "fullscreen" => keyboard_handler::click(Key::F11).await,
        "focus" => keyboard_handler::click(Key::Unicode('b')).await,
        _ => {}
    }
}