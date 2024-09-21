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
                p { "02 Oct. 2024" }
            }
        }

        introduction::slides {}

        vanilla_wc::slides {}

        rust_wc::slides {}

        dioxus_wc::slides {}

        conclusion::slides {}
    }
}
