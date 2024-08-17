use dioxus::prelude::*;

use crate::{Browser, Terminal};

#[component]
pub fn LiveCode(url: String, workdir: Option<String>, tools: Element) -> Element {
    rsx! {
        div { class: "live-code",
            Terminal { workdir }
            Browser { url, tools }
        }
    }
}
