#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct CountEvent {
    pub count: usize,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct CountRustElement {
    host: Element,
    root: ShadowRoot,
    count: Cell<usize>,
    name_el: Option<Element>,
    count_el: Option<Element>,
    listeners: Vec<EventListener>
}
