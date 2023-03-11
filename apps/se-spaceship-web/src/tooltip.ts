// Adapted from https://svelte.dev/repl/dd6754a2ad0547c5b1c1ea37c0293fef?version=3.55.1

export function tooltip(element: Element) {
  let div: HTMLDivElement | undefined;
  let title: string | undefined;

  // let showing = false;
  let toolTipElement: HTMLDivElement | undefined = undefined;

  function mouseOver(event: MouseEvent) {
    if (toolTipElement) return;

    // showing = true;

    // NOTE: remove the `title` attribute, to prevent showing the default browser tooltip
    // remember to set it back on `mouseleave`
    title = element.getAttribute("title") || element.getAttribute("data-title");
    element.removeAttribute("title");

    const rect: DOMRect = element.getBoundingClientRect();
    // console.log(rect);

    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;
    console.log(centerX, centerY);

    toolTipElement = toolTipElement ?? document.createElement("div");

    // div = document.createElement("div");
    toolTipElement.textContent = title;
    toolTipElement.className = "tooltip";
    toolTipElement.style.cssText = `
			position: absolute;
			top: ${centerY}px;
			left: ${centerX}px;
		`;
    document.body.appendChild(toolTipElement);
  }

  function mouseMove(event: MouseEvent) {
    if (!toolTipElement) return;
    // div.style.left = `${event.pageX + 5}px`;
    // div.style.top = `${event.pageY + 5}px`;
  }

  function mouseLeave() {
    if (!toolTipElement) return;

    document.body.removeChild(toolTipElement);
    toolTipElement = undefined;
    // NOTE: restore the `title` attribute
    element.setAttribute("title", title);
  }

  element.addEventListener("mouseover", mouseOver);
  element.addEventListener("mouseleave", mouseLeave);
  element.addEventListener("mousemove", mouseMove);

  return {
    destroy() {
      element.removeEventListener("mouseover", mouseOver);
      element.removeEventListener("mouseleave", mouseLeave);
      element.removeEventListener("mousemove", mouseMove);
    },
  };
}
