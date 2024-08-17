use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

mod components;
pub use self::components::*;

mod pages;
pub use self::pages::*;

mod talk;
pub use self::talk::*;

mod id;
pub use self::id::*;

mod keybindings;
pub use self::keybindings::*;

static SLIDE_ID: GlobalSignal<SlideId> = Signal::global(SlideId::default);

#[component]
pub fn App() -> Element {
    let config = || {
        RouterConfig::<Page>::default().on_update(|state| {
            let page = state.current();
            debug!(?page, "🚗 update route");
            if let Page::Live { current } = page {
                if SLIDE_ID() != current {
                    *SLIDE_ID.write() = current;
                }
            }
            None
        })
    };

    rsx! {
        Router::<Page> { config }
        if cfg!(feature = "print") {
            GithubCorner {
                href: "https://github.com/ilaborie/slides-jug-summer-camp-24",
            }
        }
    }
}
