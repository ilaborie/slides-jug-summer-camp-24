use dioxus::prelude::*;

use crate::{Part, Slide};

#[component]
pub(super) fn slides() -> Element {
    rsx! {
        Part { id: 0x50.into(),
            hgroup {
                h2 { "🥡 Conclusion" }
            }
        }

        Slide { id: 0x51.into(), title: "🙌 Yes we can!",
            article { class: "center",
                h4 { "TL;DR" }
                div { "(si votre navigateur a moins de 5 ans)" }

                p { "Rust est une solution intéressante pour faire des composants web" }
                p {
                    strong { "Essayez-le" }
                    " si vous appréciez la "
                    span { lang: "en", "type-safety" }
                    ", et acceptez un écosystème en cours de maturation"
                }
            }
        }

        Slide { id: 0x52.into(), title: "🤔 Est-ce une bonne idée ?",
            div { class: "body center",
                p { "Ça dépend !" }
                pre {
                    "dist/index.html                           1.10 kB │ gzip:  0.48 kB
dist/assets/rust_wc_bg-NzLtFgor.wasm     25.51 kB
dist/assets/dioxus_wc_bg-DB6Otv1_.wasm  413.08 kB
dist/assets/index-DorIzgSS.css            0.08 kB │ gzip:  0.09 kB
dist/assets/index-BOjFjW9c.js            52.73 kB │ gzip: 13.57 kB
"
                }
            }
        }

        Slide { id: 0x53.into(), title: "⚖️ Les pour et contre",
            div { class: "body center two-columns",
                article {
                    p {
                        strong { "✅ Oui" }
                        ", si vous"
                    }
                    p { "connaissez déjà Rust," }
                    p { "aimez le typage fort," }
                    p { "êtes prêt à développer les briques manquantes," }
                    p { "avez suffisamment de temps." }
                }
                article {
                    p {
                        strong { "🛑 Non" }
                        ", si vous"
                    }
                    p { "débutez en Rust" }
                    p { "voulez un écosystème complet, prêt à l'emploi," }
                    p { "n'avez pas de temps (prototypage)" }
                }
            }
        }

        Slide { id: 0x54.into(), title: "🔭 Aller plus loin",
            div { class: "body center",
                p {
                    "L'écosystème "
                    abbr { title: "User Interface", "UI" }
                    " de Rust est en train de se stabiliser"
                }
                p { "Le potentiel est énorme" }
                p {
                    del { "Restez à l'écoute" }
                    ins { " 🫶 Venez contribuer!" }
                }
            }
        }

        Slide { id: 0x55.into(), title: "🤗 Merci",
            div { class: "center",
                "data-theme": "light",
                p {
                    "Le code "
                    br {}
                    a {
                        class: "github",
                        href: "https://github.com/ilaborie/jug-summer-camp-24",
                        code { "ilaborie/jug-summer-camp-24" }
                    }
                }
                p {
                    "Les slides "
                    br {}
                    a {
                        class: "github",
                        href: "https://github.com/ilaborie/slides-jug-summer-camp-24",
                        code { "ilaborie/slides-jug-summer-camp-24" }
                    }
                }
                p {
                    "Vos retours constructifs sont bienvenus "
                    br {}
                    a {
                        class: "feedback",
                        href: "https://app.voxxr.in/events/jsc24",
                        code { "https://app.voxxr.in/events/jsc24" }
                    }
                }
                img { class: "ferris", src: "/slides-jug-summer-camp-24/icons/Rustacean-flat-happy.svg" }
            }
        }
    }
}
