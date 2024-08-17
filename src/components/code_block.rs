use dioxus::prelude::*;
use dioxus_web::WebEventExt as _;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub enum Language {
    #[display("typescript")]
    Typescript,

    #[display("javascript")]
    Javascript,

    #[display("rust")]
    Rust,
}

#[component]
pub fn CodeBlock(language: Language, code: String) -> Element {
    let highlight_code = move |event: Event<MountedData>| {
        let elt = event.web_event();
        Prism::highlight_element(elt);
    };

    rsx! {
        pre { class: "code-block line-numbers",

            code {
                class: "language-{language} line-numbers",
                onmounted: highlight_code,
                "{code}"
            }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    type Prism;

    #[wasm_bindgen(static_method_of = Prism, js_name = "highlightElement")]
    fn highlight_element(elt: &web_sys::Element);
}
