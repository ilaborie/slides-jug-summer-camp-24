use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};
use dioxus_web::WebEventExt as _;
use futures::channel::mpsc;
use futures::StreamExt as _;
use gloo::events::EventListener;
use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt as _;

use crate::Window;

mod bridge;
use self::bridge::{start_terminal, XtermBridge, XtermJs};

// const FONT_SIZE: u8 = 18;
// const FONT_SIZE: u8 = 20;
const FONT_SIZE: u8 = 22;
// const FONT_SIZE: u8 = 32;

const THEME: &str = include_str!("catppuccin-macchiato.json");
// const THEME: &str = include_str!("catppuccin-latte.json");

#[component]
pub fn Terminal(workdir: Option<String>) -> Element {
    let command = workdir.map(|path| TerminalCommand::WorkDir { path });

    rsx! {
        InnerTerminal { logo: "🐟", command }
    }
}

#[component]
pub fn Helix(workdir: Option<String>, file: Option<String>) -> Element {
    let command = workdir
        .zip(file)
        .map(|(workdir, file)| TerminalCommand::Helix { workdir, file });

    rsx! {
        InnerTerminal { logo: "🧬", command }
    }
}

#[component]
fn InnerTerminal(logo: String, command: Option<TerminalCommand>, font_size: Option<u8>) -> Element {
    let mut listener = use_signal(Option::<EventListener>::default);
    let title = use_signal(String::default);
    let terminal = use_signal(|| None::<XtermJs>);

    let configure_xterm = move |event: Event<MountedData>| {
        let root = event.web_event();
        init_xterm(root, terminal, title, font_size.unwrap_or(FONT_SIZE));

        let lstn = EventListener::new(root, "keyup", move |event| {
            // Do not bubble event
            event.set_cancel_bubble(true);
        });
        listener.set(Some(lstn));
    };

    use_effect(use_reactive!(|command| {
        if let Some(term) = terminal.read().as_ref() {
            if let Some(command) = command {
                info!(?command, "🏹 send command to terminal");
                let data = serde_json::to_vec(&command).expect("serialize");
                if let Err(error) = term.command(&data) {
                    error!(?error, "fail to send command");
                }
            }
        }
    }));

    rsx! {
        Window { title: "{logo} {title}", tools: None,
            div { class: "terminal", onmounted: configure_xterm }
        }
    }
}

/// Message send by the JS side
#[allow(dead_code)]
#[derive(Debug)]
enum TerminalMessage {
    Title(String),
    Resize { cols: u16, rows: u16 },
    Close,
}

/// Message send to the JS side
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "message", rename_all = "kebab-case")]
pub enum TerminalCommand {
    SetSize { rows: u16, cols: u16 },
    WorkDir { path: String },
    Helix { workdir: String, file: String },
}

fn init_xterm(
    root: &web_sys::Element,
    mut terminal: Signal<Option<XtermJs>>,
    title: Signal<String>,
    // command: &TerminalCommand,
    font: u8,
) {
    info!("🐎 Init XtermJS");
    let (tx, rx) = mpsc::unbounded();

    let bridge = XtermBridge::new(tx);
    let term = start_terminal(root, bridge, font, THEME)
        .map_err(|err| {
            error!(?err, "🚨 oops, fail to start terminal");
            err
        })
        .unwrap_throw();

    terminal.set(Some(term));

    spawn(async move { consume_terminal_messages(rx, terminal, title).await });
}

async fn consume_terminal_messages(
    mut rx: UnboundedReceiver<TerminalMessage>,
    terminal: Signal<Option<XtermJs>>,
    mut title: Signal<String>,
) {
    info!("🐙 Start receiving terminal messages");
    while let Some(message) = rx.next().await {
        debug!(?message, "🐙 Receive message");
        match message {
            TerminalMessage::Title(new_title) => {
                title.set(new_title);
            }
            TerminalMessage::Resize { cols, rows } => {
                debug!(%rows, %cols, "🌀 terminal ask resize");
                let msg = TerminalCommand::SetSize { rows, cols };
                let data = serde_json::to_vec(&msg).expect("serialize");
                if let Some(term) = terminal.read().as_ref() {
                    if let Err(error) = term.command(&data) {
                        error!(?error, "fail to send command to terminal");
                    }
                }
            }
            TerminalMessage::Close => {
                info!("🌀🚪 Terminal ask to close");
                break;
            }
        }
    }
    info!("🐙🛑 Stop receiving terminal messages");
}
