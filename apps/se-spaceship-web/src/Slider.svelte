<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let min = 0;
  export let max = 100;
  export let step = 1;
  export let pageStep = step * 10;
  export let id = null;
  export let value = min;
  export let disabled = false;

  let button: HTMLDivElement = null;
  let slider: HTMLDivElement = null;
  let track: HTMLDivElement = null;

  let sliderActive = false;
  let trackWidth: number = null;

  $: stepDiff = Math.round(min / step) * step - min;
  $: valuePrecision = value.toString().split(".")[1]
    ? value.toString().split(".")[1].length
    : 0;

  function clamp(value: number, min: number, max: number): number {
    return Math.min(Math.max(value, min), max);
  }

  const dispatch = createEventDispatcher();

  function setValue(newValue: number) {
    const saveValue = Math.min(Math.max(newValue, min), max);
    const nearestValue = Math.round(saveValue / step) * step - stepDiff;

    const newerValue = valuePrecision
      ? parseFloat(nearestValue.toFixed(valuePrecision))
      : Math.round(nearestValue);

    if (newerValue !== value) {
      value = newerValue;
      dispatch("input", { value });

      refreshSliderPosition();
    }
  }

  function updateValueFromClientX(sliderClientX: number) {
    const pointerButtonPosition =
      sliderClientX - slider.offsetLeft - button.offsetWidth / 2;

    const maxWidth = slider.offsetWidth - button.offsetWidth;

    setValue((pointerButtonPosition * (max - min)) / maxWidth + min);
  }

  function onSliderDown(event: PointerEvent) {
    if (disabled) return;

    event.stopPropagation();

    button.setPointerCapture(event.pointerId);
    sliderActive = true;
  }

  function onSliderUp(event: PointerEvent) {
    if (disabled) return;

    event.stopPropagation();

    button.releasePointerCapture(event.pointerId);
    sliderActive = false;

    dispatch("change", { value });
  }

  function onSliderMove(event: PointerEvent) {
    if (
      !sliderActive ||
      !button.hasPointerCapture(event.pointerId) ||
      disabled
    ) {
      return;
    }

    event.stopPropagation();

    const pointerX = event.clientX;

    updateValueFromClientX(pointerX);
  }

  function onTrackHit(e: MouseEvent) {
    if (disabled) return;

    e.stopPropagation();

    updateValueFromClientX(e.clientX);
  }

  function onKeyDown(e: KeyboardEvent) {
    if (disabled) return;

    let handled = true;

    switch (e.key) {
      case "ArrowLeft":
      case "ArrowDown":
        setValue(Math.max(min, value - step));
        break;
      case "ArrowRight":
      case "ArrowUp":
        setValue(Math.min(max, value + step));
        break;
      case "Home":
        setValue(min);
        break;
      case "End":
        setValue(max);
        break;
      case "PageDown":
        setValue(Math.max(min, value - pageStep));
        break;
      case "PageUp":
        setValue(Math.min(max, value + pageStep));
        break;
      default:
        handled = false;
    }

    if (handled) e.preventDefault();
  }

  $: if (value && slider) {
    value = clamp(value, min, max);
    refreshSliderPosition();
  }

  function refreshSliderPosition() {
    const newPosition =
      ((value - min) / (max - min)) * (slider.offsetWidth - button.offsetWidth);

    button.style.left = newPosition.toString() + "px";
    trackWidth = newPosition + button.offsetWidth / 2;
  }
</script>

<div
  {id}
  bind:this={slider}
  class="custom-slider"
  role="slider"
  aria-valuemin={min}
  aria-valuemax={max}
  aria-valuenow={value}
  tabindex={disabled ? -1 : 0}
  on:keydown={onKeyDown}
  on:pointerup={onTrackHit}
>
  <div class="custom-slider-background" />
  <div
    class="custom-slider-track"
    bind:this={track}
    class:disabled
    style="width: {trackWidth}px"
  />
  <div
    bind:this={button}
    class="custom-slider-button"
    class:active={sliderActive}
    class:disabled
    draggable="false"
    on:pointerdown={onSliderDown}
    on:pointermove={onSliderMove}
    on:pointerup={onSliderUp}
  />
</div>

<style>
  .custom-slider {
    position: relative;
    box-sizing: border-box;
    width: 200px;
    display: flex;
    align-items: center;
    /* touch-action: pan-y; */
  }

  .custom-slider-background {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background-color: #8c8b8b;
    box-shadow: inset 2px 2px 3px black;
    width: 100%;
    position: absolute;
    height: 4px;
    z-index: 1;
  }

  .custom-slider-track {
    width: 100%;
    height: 4px;
    z-index: 2;

    height: 12px;
    background-color: var(--ui-highlight-color);
    box-shadow: var(--inset-shadow), var(--factorio-input-outline-faint-shadow);
    border-bottom: var(--inset-border-bottom);
    position: relative;
  }

  .custom-slider-track.disabled {
    --factorio-range-slider-bg-disabled: #302f30;
    background-color: var(--factorio-range-slider-bg-disabled);
  }

  .custom-slider-button {
    user-select: none;
    --width: 20px;
    --image-width: 40;
    --image-height: 24;
    width: var(--width);
    height: calc((var(--image-height) / var(--image-width)) * var(--width));
    left: 0;
    box-shadow: 1px 1px 2px 1px rgba(0, 0, 0, 0.35);
    z-index: 3;
    background-image: url("./assets/images/slider-thumb-normal.png");
    background-size: cover;

    position: absolute;
    top: 50%;
    transform: translateY(-50%);
  }

  .custom-slider:focus-visible {
    outline: none;
  }

  .custom-slider:focus-visible .custom-slider-button:not(.disabled) {
    outline: 2px solid var(--ui-highlight-color);
  }

  .custom-slider:focus-visible .custom-slider-button:not(.disabled),
  .custom-slider-button:hover:not(:active):not(.disabled) {
    box-shadow: 0 0 3px 1px var(--ui-highlight-color);
    background-image: url("./assets/images/slider-thumb-hover.png");
  }

  .custom-slider-button:active {
    background-image: url("./assets/images/slider-thumb-active.png");
  }

  .custom-slider-button.disabled {
    background-image: url("./assets/images/slider-thumb-disabled.png");
  }
</style>
