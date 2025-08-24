use crate::{
    handlers::{action_handler, command_handler, media_handler, presentation_handler},
    websocket::{SharedOutbox, SharedOutboxExt},
};

pub async fn message_handler(msg: &str, shared_outbox: &SharedOutbox) {
    let (dispacher, arguments) = msg.split_once('#').unwrap_or_else(|| ("INVALID", ""));
    match dispacher {
        // echo
        "PING" => shared_outbox.quick_send("PONG").await,

        // Actions
        "CMD" => command_handler::command_handler(arguments.split(' ').collect()),

        // System
        "LOCK" => action_handler::lock_device(),

        // Presentation
        "PRSTN" => presentation_handler::presentation_handler(arguments).await,

        // Media
        "MEDIA" => media_handler::media_handler(arguments).await,

        // Invalid
        _ => shared_outbox.quick_send("INVALID COMMAND").await,
    }
}
