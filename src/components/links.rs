use dioxus::prelude::*;

#[component]
pub fn Mdn(path: String, children: Element) -> Element {
    rsx! {
        a {
            class: "mdn",
            href: "https://developer.mozilla.org/en-US/docs/{path}",
            {children}
        }
    }
}
