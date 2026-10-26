<script lang="ts">
  let {
    mood = "idle",
    pressed = false,
  }: {
    mood?: "idle" | "happy" | "hungry" | "tired";
    pressed?: boolean;
  } = $props();

  const file = $derived(
    mood === "happy" ? "happy" : mood === "hungry" ? "hungry" : mood === "tired" ? "tired" : "idle",
  );
</script>

<img
  class="pet-art"
  class:pressed
  src={`/pets/kuudere/${file}.png`}
  alt=""
  draggable="false"
/>

<style>
  .pet-art {
    display: block;
    width: 168px;
    height: auto;
    transform-origin: 50% 85%;
    filter: drop-shadow(0 14px 12px rgba(30, 24, 48, 0.28));
    user-select: none;
    -webkit-user-drag: none;
  }

  .pressed {
    transform: scale(0.97) translateY(2px);
    transition: transform 80ms ease-out;
  }
</style>
