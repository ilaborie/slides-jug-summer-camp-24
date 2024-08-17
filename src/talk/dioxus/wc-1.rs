use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    register_counter();
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct CountEvent {
    pub count: usize,
}

// ...
