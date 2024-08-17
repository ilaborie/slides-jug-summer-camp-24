use dioxus::prelude::*;

use crate::Cover;

mod introduction;

mod vanilla_wc;

mod rust_wc;

mod dioxus_wc;

mod conclusion;

#[component]
pub fn Talk() -> Element {
    rsx! {

        Cover {
            hgroup { class: "body center",
                "data-theme": "light",
                h1 {
                    "Et si on écrivait nos "
                    span { lang: "en", "Web components" }
                    " en Rust ?"
                }
                strong { "Igor Laborie" }
                p { "07 Sept. 2024" }
            }

            img { class: "ferris", src: "/slides-jug-summer-camp-24/icons/ferris.svg" }
        }

        introduction::slides {}

        vanilla_wc::slides {}

        rust_wc::slides {}

        dioxus_wc::slides {}

        conclusion::slides {}
    }
}
