use std::cell::Cell;

use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, Element, ShadowRoot, 
    ShadowRootInit, ShadowRootMode};

#[wasm_bindgen(module = "/src/glue.js")]
extern "C" {
    fn register(tag: &str, attributes: Vec<String>, web_component: CustomElementBuilder);
}

#[wasm_bindgen]
pub struct CustomElementBuilder;

#[wasm_bindgen]
impl CustomElementBuilder {
    pub fn inner(&self, host: &Element) -> CountRustElement {
        CountRustElement::new(host)
    }
}
// ...