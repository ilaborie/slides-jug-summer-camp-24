use dioxus::hooks::UnboundedSender;
use dioxus_logger::tracing::{error, warn};
use wasm_bindgen::prelude::*;

use super::TerminalMessage;

#[wasm_bindgen(module = "/src/components/terminal/terminal.js")]
extern "C" {

    pub type XtermJs;

    #[wasm_bindgen(method, catch)]
    pub(super) fn write(this: &XtermJs, data: &[u8]) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn clear(this: &XtermJs) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn close(this: &XtermJs) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn fit(this: &XtermJs) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn command(this: &XtermJs, data: &[u8]) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub(super) fn start_terminal(
        root: &web_sys::Element,
        bridge: XtermBridge,
        font_size: u8,
        theme: &str,
    ) -> Result<XtermJs, JsValue>;

}

#[wasm_bindgen]
pub(super) struct XtermBridge {
    #[allow(dead_code)]
    out_tx: Option<UnboundedSender<TerminalMessage>>,
}

#[allow(dead_code)]
impl XtermBridge {
    pub(super) fn new(tx: UnboundedSender<TerminalMessage>) -> Self {
        let out_tx = Some(tx);
        Self { out_tx }
    }

    fn send(&self, message: TerminalMessage) {
        let Some(tx) = &self.out_tx else {
            error!("XTerm bridge connection closed");
            return;
        };
        if tx.is_closed() {
            warn!("🤔 channel closed");
            return;
        }
        if let Err(error) = tx.unbounded_send(message) {
            error!(%error, "fail to send message");
        }
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[allow(dead_code)]
impl XtermBridge {
    pub fn title(&self, title: String) {
        let message = TerminalMessage::Title(title);
        self.send(message);
    }

    pub fn resized(&self, rows: u16, cols: u16) {
        let message = TerminalMessage::Resize { cols, rows };
        self.send(message);
    }

    pub fn close(&mut self) {
        let message = TerminalMessage::Close;
        self.send(message);
        self.out_tx.take();
    }
}
