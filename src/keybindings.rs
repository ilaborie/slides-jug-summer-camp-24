use derive_more::derive::Debug;
use dioxus::hooks::Coroutine;
use dioxus_logger::tracing::{debug, info};
use gloo::events::EventListener;
use gloo::utils::document;
use wasm_bindgen::JsCast as _;
use web_sys::KeyboardEvent;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    First,
    Last,
    NextSlide,
    PreviousSlide,
}

pub fn key_bindings(tx: Coroutine<Action>) -> EventListener {
    let document = document();
    EventListener::new(&document, "keyup", move |event| {
        let event = event.unchecked_ref::<KeyboardEvent>();
        let target = event.target().unwrap();
        info!(?target, "Event target");
        // xterm-helper-textarea
        let key = event.key();
        let action = match key.as_str() {
            " " | "ArrowDown" | "ArrowRight" | "Enter" | "PageDown" => Some(Action::NextSlide),
            "Backspace" | "ArrowUp" | "ArrowLeft" | "PageUp" => Some(Action::PreviousSlide),
            "Tab" if event.shift_key() => Some(Action::PreviousSlide),
            "Tab" if !event.shift_key() => Some(Action::NextSlide),
            "Home" => Some(Action::First),
            "End" => Some(Action::Last),
            _ => None,
        };
        if let Some(action) = action {
            tx.send(action);
        } else {
            debug!(%key, "💊 key event skipped");
        }
    })
}
