use crate::handlers::keyboard_handler;
use enigo::Key;

pub async fn media_handler(msg: &str) {
    match msg.trim() {
        "playpause" => keyboard_handler::click(Key::MediaPlayPause).await,
        "prev" => keyboard_handler::click(Key::MediaPrevTrack).await,
        "next" => keyboard_handler::click(Key::MediaNextTrack).await,
        _ => {}
    }
}