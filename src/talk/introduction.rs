use dioxus::prelude::*;

use crate::Slide;

pub(super) fn slides() -> Element {
    rsx! {
        Slide { id: 0x10.into(), title: "$> whoami",
            div { class: "body center two-columns",
                article {
                    h3 {
                        img {
                            class: "avatar",
                            src: "https://avatars.githubusercontent.com/u/111920?v=4"
                        }
                        "Igor Laborie"
                    }
                    p { "Senior backend engineer" }
                    address {
                        a {
                            class: "email",
                            href: "mailto:igor.laborie@proton.me",
                            "igor.laborie@proton.me"
                        }
                        br {}
                        a {
                            class: "email",
                            href: "mailto:igor.laborie@vector8.com",
                            "igor.laborie@vector8.com"
                        }
                    }
                }
                article {
                    h3 {
                        img { alt: "", src: "/slides-jug-summer-camp-24/vector8-logo.svg" }
                        " "
                        img { alt: "Vector8", src: "/slides-jug-summer-camp-24/vector8.svg" }
                    }
                    address {
                        a {
                            target: "_blank",
                            href: "https://www.vector8.com/",
                            "www.vector8.com"
                        }
                    }
                }
            }
        }

        Slide { id: 0x11.into(), title: "Plan",
            div { class: "body center",
                p {
                    "Construire un "
                    span { lang: "en", " Web Component " }
                    " en"
                }
                ol {
                    li { class: "natif", "Natif" }
                    li { class: "rust", "Rust" }
                    li { class: "dioxus", "Dioxus" }
                }
            }
        }
    }
}
