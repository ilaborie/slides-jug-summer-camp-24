use dioxus::launch;
use dioxus_logger::tracing::Level;

use slides_jug_summer_camp_24::App;
fn main() {
    let level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };
    dioxus_logger::init(level).expect("failed to init logger");

    launch(App);
}
