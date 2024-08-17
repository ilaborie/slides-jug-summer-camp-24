use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

use crate::{SlideId, Talk, SLIDE_ID};

#[component]
pub fn Live(current: SlideId) -> Element {
    use_effect(move || {
        debug!(?current, "🌳 Live");
        *SLIDE_ID.write() = current;
    });

    rsx!(
        Talk {}
    )
}
