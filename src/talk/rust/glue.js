export function register(tag, attributes, wc) {
  customElements.define(
    tag,
    // This is an anonymous class
    class extends HTMLElement {
      constructor() {
        super();
        this.inner = wc.inner(this);
      }

      connectedCallback() {
        this.inner.connectedCallback();
      }
      disconnectedCallback() {
        this.inner.disconnectedCallback();
      }

      static observedAttributes = attributes;
      attributeChangedCallback(name, oldValue, newValue) {
        this.inner.attributeChangedCallback(name, oldValue, newValue);
      }
    },
  );
}
