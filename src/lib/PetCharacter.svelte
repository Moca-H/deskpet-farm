<script lang="ts">
  import { onMount } from "svelte";

  let {
    mood = "idle",
    pressed = false,
  }: {
    mood?: "idle" | "happy" | "hungry" | "tired";
    pressed?: boolean;
  } = $props();

  const FRAMES = 16;
  let frame = $state(1);

  onMount(() => {
    const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    if (reduce) return;
    const t = setInterval(() => {
      frame = frame >= FRAMES ? 1 : frame + 1;
    }, mood === "tired" ? 180 : 90);
    return () => clearInterval(t);
  });

  const src = $derived(`/pets/idle/${String(frame).padStart(2, "0")}.png`);
</script>

<img
  class="pet-art"
  class:pressed
  class:hungry={mood === "hungry"}
  class:tired={mood === "tired"}
  class:happy={mood === "happy"}
  {src}
  alt=""
  draggable="false"
/>

<style>
  .pet-art {
    display: block;
    width: 210px;
    height: auto;
    transform-origin: 50% 90%;
    filter: drop-shadow(0 16px 14px rgba(40, 30, 50, 0.28));
    user-select: none;
    -webkit-user-drag: none;
  }

  .pressed {
    transform: scale(0.96);
    transition: transform 80ms ease-out;
  }

  .hungry {
    filter: drop-shadow(0 16px 14px rgba(40, 30, 50, 0.28)) saturate(0.85);
  }

  .tired {
    filter: drop-shadow(0 16px 14px rgba(40, 30, 50, 0.22)) brightness(0.92);
  }

  .happy {
    filter: drop-shadow(0 16px 14px rgba(40, 30, 50, 0.28)) saturate(1.08);
  }

  @media (prefers-reduced-motion: reduce) {
    .pressed {
      transition: none;
    }
  }
</style>
