use dioxus::prelude::*;

use crate::{Part, Slide};

#[component]
pub(super) fn slides() -> Element {
    rsx! {
        Part { id: 0x30.into(),
            hgroup {
                h2 { class: "rust", "Rust" }
                p {
                    span { lang: "en", "Web component" }
                    " en WASM avec Rust "
                }
            }
        }

        Slide { id: 0x31.into(), title: "Web Assembly (WASM)",
            div { class: "body center",
                p {
                    "Format binaire, facile à "
                    span { lang: "en", "parser" }
                    ", et à compiler"
                }
                p { "Performance proche du code natif" }
                p { "Peut aussi être utilisé en dehors Web" }
                p {
                    a { class: "wasm", href: "https://developer.mozilla.org/en-US/docs/WebAssembly",
                        "WebAssembly"
                    }
                }
            }
        }

        Slide { id: 0x32.into(), title: "Rust",
            div { class: "body center",
                blockquote {
                    "Un langage qui offre à tous le pouvoir de construire des logiciels fiables et efficaces."
                    footer {
                        cite {
                            a { href: "https://www.rust-lang.org/fr", "-- Site officiel de Rust" }
                        }
                    }
                }
                p {
                    "Pas de "
                    abbr { title: "Garbage Collector", "GC" }
                    ", mais SANS soucis de mémoire"
                }
                p {
                    "Interopérable avec "
                    abbr { title: "Foreign function interface", "FFI" }
                    ","
                    br {}
                    "par exemple "
                    a { href: "https://pyo3.rs/", "PyO3" }
                    " pour Python"
                }
            }
        }

        Slide { id: 0x33.into(), title: "Rust - Ownership & Borrowing",
            article { class: "center",
                dl {
                    dt {
                        span { lang: "en", "Ownership" }
                    }
                    dd {
                        "chaque valeur a un propriétaire,"
                        br {}
                        "elle est "
                        span { lang: "en", "droppée" }
                        " lors que le propriétaire sort du "
                        span { lang: "en", "scope" }
                        br {}
                        "on peut transmettre la propriété ("
                        span { lang: "en", "move" }
                        ")"
                    }
                    dt {
                        span { lang: "en", "Borrowing" }
                    }
                    dd {
                        code { "&T" }
                        " immutable, plusieurs lecteurs, pas d'auteurs"
                        br {}
                        code { "&mut T" }
                        " mutable, un seul auteur, pas de lecteurs"
                    }
                }
            }
        }

        {code()},

        Slide { id: 0x3e.into(), title: "Rust - bilan",
            article { class: "center",
                p {
                    a {
                        class: "book",
                        href: "https://rustwasm.github.io/docs/book/",
                        "Le live 'Rust 🦀 and WebAssembly 🕸️'"
                    }
                }
                p {
                    a {
                        class: "tool",
                        href: "https://rustwasm.github.io/docs/wasm-pack/",
                        code { "wasm-pack" }
                    }
                    "le couteau suisse pour faire du WASM en Rust"
                }
                p {
                    a {
                        class: "crate",
                        href: "https://rustwasm.github.io/wasm-bindgen/",
                        code { "wasm-bindgen" }
                    }
                    "interopérabilité pour le code Rust en WASM"
                }
                p {
                    a {
                        class: "crate",
                        href: "https://docs.rs/web-sys/latest/web_sys/",
                        code { "web-sys" }
                    }
                    "les "
                    abbr { title: "Application Programming Interface", "API" }
                    " Web en Rust"
                }
                p {
                    a {
                        class: "crate",
                        href: "https://docs.rs/gloo/latest/gloo/",
                        code { "gloo" }
                    }
                    "simplifie des API"
                    br {}
                    "par exemple"
                    code { "fetch" }
                }
            }
        }
    }
}

#[cfg(not(feature = "print"))]
fn code() -> Element {
    use crate::{LiveCode, SlideKind};

    rsx! {
        Slide { id: 0x34.into(), kind: SlideKind::Full, title: "Rust - livecode",
            LiveCode { url: "http://localhost:5173", tools: None }
        }
    }
}

#[cfg(feature = "print")]
fn code() -> Element {
    use crate::{CodeBlock, Language, SlideId};

    let codes = [
        (Language::Rust, include_str!("rust/final-1.rs")),
        (Language::Javascript, include_str!("rust/glue.js")),
        (Language::Rust, include_str!("rust/final-2.rs")),
        (Language::Rust, include_str!("rust/final-3.rs")),
        (Language::Rust, include_str!("rust/final-4.rs")),
        (Language::Rust, include_str!("rust/final-5.rs")),
        (Language::Rust, include_str!("rust/final-6.rs")),
    ];
    let len = codes.len();
    let codes = codes
        .iter()
        .enumerate()
        .map(|(idx, (lang, code))| {
            let id = SlideId::from(0x34 + idx as u8);
            let title = format!("Rust - code {}/{}", idx + 1, len);

            (id, title, *lang, code)
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
