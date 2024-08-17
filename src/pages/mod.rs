use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use futures::StreamExt as _;
use gloo::events::EventListener;

use crate::{key_bindings, Action, SlideId, SLIDE_ID};

mod live;
pub use self::live::*;

#[derive(Debug, Clone, Routable)]
#[rustfmt::skip]
pub enum Page {
    #[redirect("/", || Self::Live {current: SlideId::default()})]


    // #[nest("/slides-jug-summer-camp-24")]
    #[redirect("/", || Self::Live {current: SlideId::default()})]
    #[layout(CommonLayout)]
        #[route("/live/:current")]
        Live { current: SlideId },

        #[route("/lighttable")]
        LightTable,

}

#[component]
fn CommonLayout() -> Element {
    let mut listener = use_signal(Option::<EventListener>::default);
    let tx = use_coroutine(|mut rx| async move {
        let nav = navigator();
        while let Some(action) = rx.next().await {
            info!(?action, "🐳 revieve data");
            let current = SLIDE_ID();
            let next = match action {
                Action::First => SlideId::first(),
                Action::Last => SlideId::last(),
                Action::PreviousSlide => current.previous(),
                Action::NextSlide => current.next(),
            };
            if let Some(next) = next {
                nav.push(Page::Live { current: next });
            }
        }
    });
    use_effect(move || {
        let lstn = key_bindings(tx);
        listener.set(Some(lstn));
    });

    rsx! {
        Outlet::<Page> {}
    }
}

#[component]
fn LightTable() -> Element {
    rsx! { "TODO" }
}
