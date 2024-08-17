use dioxus::prelude::*;

#[component]
pub fn GithubCorner(href: String) -> Element {
    let svg = include_str!("github_corner.svg");

    rsx! {
        a {
            href,
            target: "_blank",
            class: "github-corner",
            aria_label: "View source on GitHub",
            dangerous_inner_html: svg,
        }
    }
}
