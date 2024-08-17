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
    count: Rc<Cell<usize>>, // interior mutability pattern
    name_el: Option<Element>,
    count_el: Option<Element>,
    listeners: Vec<EventListener>,
}
