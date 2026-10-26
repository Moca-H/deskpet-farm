<script lang="ts">
  let {
    mood = "idle",
    pressed = false,
  }: {
    mood?: "idle" | "happy" | "hungry" | "tired";
    pressed?: boolean;
  } = $props();

  const frames: Record<"idle" | "happy" | "hungry" | "tired", string[]> = {
    idle: ["idle-1"],
    happy: ["happy-1"],
    hungry: ["hungry-1"],
    tired: ["tired-1"],
  };

  const src = $derived(`/pets/pixel/${frames[mood][0]}.png`);
</script>

<img class="pet-art" class:pressed class:happy={mood === "happy"} src={src} alt="" draggable="false" />

<style>
  .pet-art {
    display: block;
    width: 132px;
    height: auto;
    transform-origin: 50% 100%;
    filter: drop-shadow(0 10px 12px rgba(40, 24, 8, 0.22));
    user-select: none;
    -webkit-user-drag: none;
    transition: transform 120ms ease;
  }

  .happy {
    animation: bob 900ms ease-in-out infinite;
  }

  .pressed {
    transform: translateY(4px) scale(0.98);
    animation: none;
  }

  @keyframes bob {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-4px);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .happy {
      animation: none;
    }
  }
</style>
