use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let mut count_event = use_signal(Option::<usize>::default);
    let on_count = move |event: CountEvent| {
        count_event.set(Some(event.count));
    };

    rsx! {
        div { class: "container",
            div { display: "flex", align_items: "center", gap: "1rem",
                Counter { name: "plop", on_count }
                code {
                    if let Some(count) = count_event() {
                        "{count}"
                    }
                }
            }
        }
    }
}
// ...