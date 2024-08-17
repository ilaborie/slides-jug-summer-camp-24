use dioxus::prelude::*;

#[component]
pub fn Window(title: String, tools: Element, children: Element) -> Element {
    rsx! {
        div { class: "window",
            header {
                div { class: "left-tools",
                    div { class: "close" }
                    div { class: "minify" }
                    div { class: "expand" }
                }
                div { class: "title", "{title}" }
                div { class: "right-tool", {tools} }
            }
            div { class: "body", {children} }
            footer {}
        }
    }
}
