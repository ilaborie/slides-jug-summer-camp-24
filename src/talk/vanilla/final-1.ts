type CountEvent = { count: number };

class CounterElement extends HTMLElement {
  private root: ShadowRoot;
  private count = 0;
  private nameEl?: Element;
  private countEl?: Element;

  constructor() {
    super();
    this.root = this.attachShadow({ mode: "closed" });
  }
  //...
}
