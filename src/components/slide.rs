use derive_more::derive::Debug;
use dioxus::prelude::*;

use crate::{SlideId, SLIDE_ID};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, derive_more::Display)]
pub enum SlideKind {
    #[display("cover")]
    Cover,
    #[display("standard")]
    #[default]
    Standard,
    #[display("part")]
    Part,
    #[display("full")]
    Full,
}

#[component]
pub fn Slide(
    id: SlideId,
    kind: Option<SlideKind>,
    title: Option<String>,
    children: Element,
) -> Element {
    let current = SLIDE_ID();
    // info!(%id, "Render");
    let kind = kind.unwrap_or_default();

    let relation = current
        .relation(&id)
        .map(|it| it.to_string())
        .unwrap_or_default();

    rsx! {
        section { "data-slide-id": "{id}", class: "slide {kind} {relation}",

            header {
                if let Some(title) = title {
                    h3 { class: "container", "{title}" }
                }
            }

            {children},

            footer {
                nav { class: "container",
                    ul {
                        li {
                            span { class: "hashtag" }
                        }
                    }
                    ul {}
                    ul {
                        li {
                            span { class: "speaker" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Cover(children: Element) -> Element {
    rsx! {
        Slide { id: 0.into(), kind: SlideKind::Cover, {children} }
    }
}

#[component]
pub fn Part(id: SlideId, children: Element) -> Element {
    rsx! {
        Slide { id, kind: SlideKind::Part, {children} }
    }
}
