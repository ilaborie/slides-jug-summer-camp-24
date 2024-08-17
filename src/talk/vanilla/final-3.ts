class CounterElement extends HTMLElement {
  // ...
  static observedAttributes = ["name"];

  attributeChangedCallback(name: string, _oldValue: string, newValue: string) {
    if (name === "name" && this.nameEl) {
      this.nameEl.textContent = newValue;
    }
  }
}

customElements.define("js-counter", CounterElement);
