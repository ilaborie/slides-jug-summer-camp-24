class CounterElement extends HTMLElement {
  //...
  connectedCallback() {
    this.name = this.getAttribute("name") || "";
    this.root.innerHTML = `
      <style>${counterCss}</style>
      <label>${name}</label>
      <button>${this.count}</button>
    `;
    this.nameEl = this.root.querySelector("label")!;
    this.countEl = this.root.querySelector("button")!;
    this.countEl.addEventListener("click", () => {
      const count = ++this.count;
      this.countEl!.textContent = `${this.count}`;
      const event = new CustomEvent<CountEvent>("count", {
        detail: { count },
      });
      this.dispatchEvent(event);
    });
  }
  //...
}
