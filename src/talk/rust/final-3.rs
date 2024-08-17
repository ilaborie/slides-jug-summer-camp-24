#[wasm_bindgen]
impl CountRustElement {
    #[wasm_bindgen(constructor)]
    pub fn new(host: &Element) -> Self {
        let root = host
            .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Closed))
            .unwrap_throw();

        Self {
            host: host.clone(),
            root,
            count: Rc::default(),
            name_el: None,
            count_el: None,
            listeners: vec![],
        }
    }
    // ...
}
