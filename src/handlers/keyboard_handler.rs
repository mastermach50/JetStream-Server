use enigo::{Enigo, Key, Keyboard, Settings};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static ENIGO: Lazy<Mutex<Enigo>> =
    Lazy::new(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()));

pub async fn click(key: Key) {
    ENIGO
        .lock()
        .await
        .key(key, enigo::Direction::Click)
        .unwrap();
}
