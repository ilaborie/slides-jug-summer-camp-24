use dioxus::prelude::*;

use crate::{Part, Slide};

#[component]
pub(super) fn slides() -> Element {
    rsx! {
        Part { id: 0x40.into(),
            hgroup {
                h2 { class: "dioxus", "Dioxus" }
                p {
                    span { lang: "en", "Web component" }
                    " en WASM avec Rust et Dioxus"
                }
            }
        }

        Slide { id: 0x41.into(), title: "Dioxus",
            div { class: "body center",
                ul { class: "center",
                    li {
                        span { lang: "en", "fullstack" }
                        ", "
                        span { lang: "en", "crossplatform" }
                        ", super rapide, complétement typé"
                    }
                    li {
                        "orienté Web, mais il existe des rendus alternatifs,"
                        br {}
                        abbr { title: "Terminal User Interface", "TUI" }
                        ", natif"
                    }
                    li { "inspiré par React, avec des signaux, et du Virtual DOM" }
                    li {
                        a {
                            class: "ycombinator",
                            href: "https://www.ycombinator.com/companies/dioxus-labs",
                            "compagnie financée"
                        }
                    }
                }

                div { class: "links", font_size: "120%",
                    a { class: "dioxus", href: "https://dioxuslabs.com/", "https://dioxuslabs.com" }

                    a {
                        class: "discord",
                        href: "https://discord.com/invite/XgGxMSkvUM",
                        "Discord"
                    }
                }
            }
        }

        {code()},

        Slide { id: 0x4f.into(), title: "Dioxus - bilan",
            article { class: "center",
                ul {
                    li {
                        "bon "
                        abbr { title: "Developer eXperience", "DX" }
                        " avec Dioxus"
                    }
                    li { "... quand vous êtes familiarisés avec les signaux" }
                    li {
                        a {
                            class: "crate",
                            href: "https://github.com/ilaborie/dioxus-web-component",
                            code { "dioxus-web-component" }
                        }
                        "supprime le code fastidieux"
                    }
                    li {
                        "masque (une grande partie de) la complexité de "
                        code { "wasm-bindgen" }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "print"))]
fn code() -> Element {
    use crate::{LiveCode, SlideKind};

    let mut port = use_signal(|| 5173_u16);

    let toggle_port = move |_| {
        let new_port = if port() == 5173 { 8080 } else { 5173 };
        port.set(new_port);
    };

    let tools = rsx! {
        input {
            r#type: "checkbox",
            role: "switch",
            font_size: "100%",
            oninput: toggle_port
        }
    };

    rsx! {
        Slide { id: 0x42.into(), kind: SlideKind::Full, title: "Dioxus - livecode",
            LiveCode { url: "http://localhost:{port}", tools }
        }
    }
}

#[cfg(feature = "print")]
fn code() -> Element {
    use crate::{CodeBlock, Language, SlideId};

    let codes = [
        include_str!("dioxus/wc-1.rs"),
        include_str!("dioxus/wc-2.rs"),
    ];
    let len = codes.len();
    let codes = codes
        .iter()
        .enumerate()
        .map(|(idx, code)| {
            let id = SlideId::from(0x42 + idx as u8);
            let title = format!("Dioxus - code {}/{}", idx + 1, len);

            (id, title, Language::Rust, code)
        })
        .collect::<Vec<_>>();

    rsx! {
        for (id , title , language , code) in codes.into_iter() {
            Slide { id, title,
                CodeBlock { language, code }
            }
        }
    }
}
