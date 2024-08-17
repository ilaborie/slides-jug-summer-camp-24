#[wasm_bindgen]
impl CountRustElement {
    // ...
    #[wasm_bindgen(js_name = "disconnectedCallback")]
    pub fn disconnected_callback(&mut self) {
        self.listeners.clear();
        self.name_el.take();
        self.count_el.take();
        self.root.set_inner_html("");
    }
    
    #[wasm_bindgen(js_name = "attributeChangedCallback")]
    pub fn attribute_changed_callback(
        &mut self,
        name: &str,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "name" {
            if let Some(el) = &self.name_el {
                el.set_text_content(new_value.as_deref());
            }
        }
    }
}

#[wasm_bindgen(start)]
fn start() {
    register("rs-counter", vec!["name".to_string()], CustomElementBuilder);
}
