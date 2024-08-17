use dioxus::prelude::*;

use crate::{Mdn, Part, Slide};

#[component]
pub(super) fn slides() -> Element {
    rsx! {
        Part { id: 0x20.into(),
            hgroup {
                h2 { class: "natif", "Natif" }
                p {
                    span { lang: "en", "Web component" }
                    " sans frameworks"
                }
            }
        }

        Slide { id: 0x21.into(), title: "Natif - standards",
            article { class: "center",
                dl {
                    dt {
                        Mdn { path: "Web/API/Web_components/Using_custom_elements",
                            span { lang: "en", "Custom Elements" }
                        }
                    }
                    dd {
                        "créer nos propres balises "
                        abbr { title: "Hyper Text Markup Language", "HTML" }
                    }

                    dt {
                        Mdn { path: "Web/API/Web_components/Using_shadow_DOM",
                            span { lang: "en", "Shadow DOM" }
                        }
                    }
                    dd {
                        "encapsulation du "
                        abbr { title: "Document Object Model", "DOM" }
                        " & "
                        abbr { title: "Cascading Style Sheets", "CSS" }
                    }

                    dt { class: "dimmed",
                        Mdn { path: "Web/API/Web_components/Using_templates_and_slots",
                            span { lang: "en", "HTML template" }
                        }
                    }
                    dd { class: "dimmed", "modèles réutilisables" }
                }
            }
        }

        {code()},

        Slide { id: 0x2e.into(), title: "Natif - bilan",
            article { class: "center",
                p {
                    "Paramètres: attributs HTML"
                    br {}
                    "☞ Prévoir une conversion depuis une "
                    code { "string" }
                }
                p {
                    "Notification"
                    br {}
                    Mdn { path: "Web/API/CustomEvent",
                        code { "CustomEvent" }
                    }
                }
                p { class: "dimmed",
                    "Accès aux méthodes, "
                    br {}
                    " et aux attributs personnalisés"
                    br {}
                    Mdn { path: "Web/JavaScript/Reference/Functions/get",
                        code { "get" }
                    }
                    ", "
                    Mdn { path: "Web/JavaScript/Reference/Functions/set",
                        code { "set" }
                    }
                }
            }
        }

        Slide { id: 0x2f.into(), title: "Natif - aller plus loin",
            div { class: "body center",
                p {
                    "Framework / lib. / compiler"
                    br {}
                    a { class: "lit", href: "https://lit.dev/", "Lit" }
                    ", "
                    a { class: "stencil", href: "https://stenciljs.com/", "Stencil" }
                    ", ..."
                }
                p {
                    "Personnalisation du style"
                    br {}
                    Mdn { path: "Web/CSS/Using_CSS_custom_properties", "CSS variables" }
                    br {}
                    Mdn { path: "Web/CSS/::part",
                        "le pseudo-élément "
                        code { "::part" }
                    }
                }
                p {
                    "Formulaire"
                    br {}
                    Mdn { path: "Web/API/ElementInternals",
                        code { "ElementInternals" }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "print"))]
fn code() -> Element {
    use crate::{LiveCode, SlideKind};

    rsx! {
        Slide { id: 0x22.into(), kind: SlideKind::Full, title: "Natif - livecode",
            LiveCode { url: "http://localhost:5173", tools: None }
        }
    }
}

#[cfg(feature = "print")]
fn code() -> Element {
    use crate::{CodeBlock, Language, SlideId};

    let codes = [
        include_str!("vanilla/final-1.ts"),
        include_str!("vanilla/final-2.ts"),
        include_str!("vanilla/final-3.ts"),
    ];
    let len = codes.len();
    let codes = codes
        .iter()
        .enumerate()
        .map(|(idx, code)| {
            let id = SlideId::from(0x22 + idx as u8);
            let title = format!("Natif - code {}/{}", idx + 1, len);

            (id, title, Language::Typescript, code)
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
