// ... Register click listener
let count = self.count.clone();
let count_el = el.clone();
let host = self.host.clone();

let listener = EventListener::new(el, "click", move |_event| {
    // Update counter
    let new_count = count.get() + 1;
    count.set(new_count);
    count_el.set_text_content(Some(&new_count.to_string()));

    // Sending a custom event
    let mut init = CustomEventInit::new();
    let detail = CountEvent { count: new_count };
    init.detail(&detail.into());
    let event = CustomEvent::new_with_event_init_dict("count", &init).unwrap_throw();
    host.dispatch_event(&event).unwrap_throw();
});
self.listeners.push(listener);
