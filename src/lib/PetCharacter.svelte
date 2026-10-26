<script lang="ts">
  /** Original chibi companion — cool blue twin-tail maid vibe (not any copyrighted character). */
  let {
    mood = "idle",
    pressed = false,
  }: {
    mood?: "idle" | "happy" | "hungry" | "tired";
    pressed?: boolean;
  } = $props();
</script>

<div
  class="stage"
  class:pressed
  class:happy={mood === "happy"}
  class:hungry={mood === "hungry"}
  class:tired={mood === "tired"}
  aria-hidden="true"
>
  <svg class="body" viewBox="0 0 160 200" width="160" height="200">
    <defs>
      <linearGradient id="hair" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#c5d0ff" />
        <stop offset="55%" stop-color="#7f93e6" />
        <stop offset="100%" stop-color="#5a6fc4" />
      </linearGradient>
      <linearGradient id="dress" x1="0" y1="0" x2="1" y2="1">
        <stop offset="0%" stop-color="#3d4a7a" />
        <stop offset="100%" stop-color="#1f2748" />
      </linearGradient>
      <radialGradient id="cheek" cx="50%" cy="50%" r="50%">
        <stop offset="0%" stop-color="#ffb3c7" stop-opacity="0.9" />
        <stop offset="100%" stop-color="#ffb3c7" stop-opacity="0" />
      </radialGradient>
    </defs>

    <!-- shadow -->
    <ellipse class="shadow" cx="80" cy="188" rx="38" ry="7" fill="rgba(30,40,80,0.22)" />

    <!-- twin tails -->
    <path
      class="tail left"
      d="M48 78 C18 95 12 140 28 168 C36 150 44 120 52 96 Z"
      fill="url(#hair)"
    />
    <path
      class="tail right"
      d="M112 78 C142 95 148 140 132 168 C124 150 116 120 108 96 Z"
      fill="url(#hair)"
    />

    <!-- body / dress -->
    <path
      d="M56 118 C56 108 68 102 80 102 C92 102 104 108 104 118 L108 158 C108 168 96 174 80 174 C64 174 52 168 52 158 Z"
      fill="url(#dress)"
    />
    <!-- apron -->
    <path
      d="M66 122 C70 118 90 118 94 122 L96 156 C90 160 70 160 64 156 Z"
      fill="#f4f7ff"
      opacity="0.95"
    />
    <circle cx="80" cy="128" r="3.2" fill="#e8a0b8" />

    <!-- arms -->
    <path d="M54 124 C42 132 40 148 48 152 C52 144 56 134 58 128 Z" fill="#ffe4d4" />
    <path d="M106 124 C118 132 120 148 112 152 C108 144 104 134 102 128 Z" fill="#ffe4d4" />

    <!-- head -->
    <ellipse cx="80" cy="78" rx="34" ry="36" fill="#ffe8dc" />
    <!-- bangs -->
    <path
      d="M48 72 C52 42 108 42 112 72 C104 58 90 52 80 52 C70 52 56 58 48 72 Z"
      fill="url(#hair)"
    />
    <path d="M62 58 C68 70 74 74 80 74 C76 62 70 56 62 58 Z" fill="#6b7fd0" opacity="0.35" />

    <!-- eyes -->
    <g class="eyes">
      <ellipse class="eye" cx="66" cy="82" rx="7.5" ry="9" fill="#2a3558" />
      <ellipse class="eye" cx="94" cy="82" rx="7.5" ry="9" fill="#2a3558" />
      <circle cx="63.5" cy="78.5" r="2.2" fill="#fff" />
      <circle cx="91.5" cy="78.5" r="2.2" fill="#fff" />
      <circle cx="68.5" cy="85" r="1.1" fill="#9ad0ff" opacity="0.85" />
      <circle cx="96.5" cy="85" r="1.1" fill="#9ad0ff" opacity="0.85" />
    </g>

    <!-- brows / mouth -->
    <path d="M58 70 Q66 67 72 70" fill="none" stroke="#4a5680" stroke-width="1.4" stroke-linecap="round" />
    <path d="M88 70 Q94 67 102 70" fill="none" stroke="#4a5680" stroke-width="1.4" stroke-linecap="round" />
    <path class="mouth" d="M74 96 Q80 100 86 96" fill="none" stroke="#c47a8a" stroke-width="1.6" stroke-linecap="round" />

    <!-- blush -->
    <ellipse cx="58" cy="92" rx="7" ry="4" fill="url(#cheek)" />
    <ellipse cx="102" cy="92" rx="7" ry="4" fill="url(#cheek)" />

    <!-- hair ribbons -->
    <circle cx="46" cy="86" r="5" fill="#e8a0b8" />
    <circle cx="114" cy="86" r="5" fill="#e8a0b8" />
  </svg>

  {#if mood === "happy"}
    <span class="fx heart">♡</span>
    <span class="fx heart delay">♡</span>
  {/if}
</div>

<style>
  .stage {
    position: relative;
    width: 160px;
    height: 200px;
    margin: 0 auto;
    transform-origin: 50% 90%;
    animation: bob 2.8s ease-in-out infinite;
    will-change: transform;
  }

  .stage.pressed {
    animation: none;
    transform: scale(0.94);
    transition: transform 90ms ease-out;
  }

  .stage.tired {
    animation-duration: 4.2s;
    filter: saturate(0.85) brightness(0.96);
  }

  .stage.hungry .mouth {
    d: path("M74 98 Q80 94 86 98");
  }

  .body {
    display: block;
    overflow: visible;
    filter: drop-shadow(0 14px 18px rgba(40, 50, 100, 0.28));
  }

  .tail.left,
  .tail.right {
    transform-origin: 80px 80px;
    animation: sway 3.2s ease-in-out infinite;
  }

  .tail.right {
    animation-delay: -1.1s;
  }

  .eyes {
    transform-origin: 80px 82px;
    animation: blink 5.5s infinite;
  }

  .fx.heart {
    position: absolute;
    left: 108px;
    top: 48px;
    color: #e8a0b8;
    font-size: 18px;
    animation: floatHeart 900ms ease-out forwards;
    pointer-events: none;
  }

  .fx.heart.delay {
    left: 118px;
    top: 62px;
    animation-delay: 120ms;
    font-size: 14px;
  }

  @keyframes bob {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-5px);
    }
  }

  @keyframes sway {
    0%,
    100% {
      transform: rotate(0deg);
    }
    50% {
      transform: rotate(3deg);
    }
  }

  @keyframes blink {
    0%,
    92%,
    100% {
      transform: scaleY(1);
    }
    94%,
    96% {
      transform: scaleY(0.12);
    }
  }

  @keyframes floatHeart {
    0% {
      opacity: 0;
      transform: translateY(6px) scale(0.6);
    }
    30% {
      opacity: 1;
    }
    100% {
      opacity: 0;
      transform: translateY(-18px) scale(1);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .stage,
    .tail.left,
    .tail.right,
    .eyes,
    .fx.heart {
      animation: none !important;
    }
  }
</style>
