use dioxus::prelude::*;

use crate::Window;

#[component]
pub fn Browser(url: String, tools: Element) -> Element {
    rsx! {
        Window { title: "🦊 {url}", tools,
            iframe { class: "browser", src: "{url}" }
        }
    }
}
