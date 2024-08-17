use std::cell::Cell;
use std::rc::Rc;

use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, Element, ShadowRoot,
	ShadowRootInit, ShadowRootMode};

#[wasm_bindgen(module = "/src/glue.js")]
extern "C" {
    fn register(tag: &str, attributes: Vec<String>, builder: Builder);
}

#[wasm_bindgen]
pub struct Builder;

#[wasm_bindgen]
impl Builder {
    pub fn inner(&self, host: &Element) -> CountRustElement {
        CountRustElement::new(host)
    }
}
// ...
