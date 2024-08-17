// ...
#[web_component(
    tag = "dx-counter",
    style = InjectedStyle::css(include_str!("counter.css"))
)]
pub fn Counter(name: Option<String>, on_count: EventHandler<CountEvent>) -> Element {
    let mut count = use_signal(usize::default);
    let name = name.unwrap_or_default();
    let onclick = move |_event| {
        count += 1;
        on_count.call(CountEvent { count: count() });
    };

    rsx! {
        label { "{name}" }
        button { onclick, "{count}" }
    }
}
