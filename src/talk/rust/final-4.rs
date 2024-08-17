impl CountRustElement {
    #[wasm_bindgen(js_name = "connectedCallback")]
    pub fn connected_callback(&mut self) {
        // Set content (shadowed)
        let css = include_str!("counter.css");
        let name = self.host.get_attribute("name").unwrap_or_default();
        let html = format!(
            "
                <style>{css}</style>
                <label>{name}</label>
                <button>{}</button>
            ",
            self.count.get()
        );
        self.root.set_inner_html(&html);
        self.name_el = self.root.query_selector("label").unwrap_or_default();
        self.count_el = self.root.query_selector("button").unwrap_or_default();

        if let Some(el) = &self.count_el {
            // Register click listener ...
        }
    }
}
