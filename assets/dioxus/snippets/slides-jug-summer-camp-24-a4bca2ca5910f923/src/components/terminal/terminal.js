export function start_terminal(root, bridge, fontSize, rawTheme) {
  // console.log("💊 Start terminal");
  const theme = JSON.parse(rawTheme);

  // Configure
  const term = new Terminal({
    fontFamily: "'Inconsolata Nerd Font Mono'",
    fontSize,
    cursorBlink: true,
    fontWeightBold: "normal",
    macOptionClickForcesSelection: true,
    allowProposedApi: true,
    scrollback: 0,
    cols: 80,
    rows: 24,
    theme,
  });

  // Open
  // console.log("💊 open terminal");
  term.open(root);

  // Addon
  // console.log("💊 add terminal addons");
  term.loadAddon(new ClipboardAddon.ClipboardAddon());
  term.loadAddon(new LigaturesAddon.LigaturesAddon());
  term.loadAddon(new WebLinksAddon.WebLinksAddon());
  term.loadAddon(new Unicode11Addon.Unicode11Addon());
  const fitAddon = new FitAddon.FitAddon();
  term.loadAddon(fitAddon);

  const ws = new WebSocket("ws://localhost:8000/");
  const result = new XTermJs(term, fitAddon, ws);

  // ws.onclose = () => {
  //   bridge.close();
  // };

  ws.onopen = () => {
    let attach = new AttachAddon.AttachAddon(ws);
    term.loadAddon(attach);

    result.flushCommands();
  };

  // Register terminal listeners
  term.onTitleChange((title) => bridge.title(title));
  term.onResize(({ rows, cols }) => {
    // console.log("🌳 resize to", { rows, cols });
    return bridge.resized(rows, cols);
  });

  // Observe size
  const observer = new ResizeObserver((entries) => {
    // console.log("Resize detected", entries);
    result.fit();
  });
  observer.observe(root);

  return result;
}

class XTermJs {
  constructor(term, fitAddon, ws) {
    this._term = term;
    this._fitAddon = fitAddon;
    this._ws = ws;
    this._commands = [];
  }

  flushCommands() {
    if (this._commands.length === 0) {
      return;
    }

    // console.log("🧺 flushing commands");
    this._commands
      .splice(0, this._commands.length)
      .forEach((data) => this.command(data));
  }

  command(data) {
    if (this._ws.readyState === WebSocket.OPEN) {
      // console.log(
      //   "🏹 Send a command to terminal",
      //   typeof data,
      //   new TextDecoder().decode(data),
      // );
      this._ws.send(data);
    } else {
      // console.warn("🧦 not open keep command");
      // XXX need to clone the data
      this._commands.push(new Uint8Array(data));
    }
  }

  write(data) {
    this._term.write(data);
  }

  fit() {
    this._fitAddon.fit();
  }

  clear() {
    console.log("🧹 clear");
    this._term.clear();
  }

  close() {
    console.log("🛑 close");
    this._term.dispose();
  }
}
