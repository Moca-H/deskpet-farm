<script lang="ts">
  const plots = Array.from({ length: 9 }, (_, i) => ({
    id: i,
    unlocked: i < 4,
  }));
</script>

<main class="farm">
  <header>
    <h1>deskpet farm</h1>
    <p>3×3 地块占位 · 前 4 格已解锁 · 玩法规则见 docs/GAMEPLAY.md</p>
  </header>

  <section class="grid" aria-label="plots">
    {#each plots as plot}
      <div class={plot.unlocked ? "plot" : "plot locked"}>
        {#if plot.unlocked}
          <span>空地</span>
        {:else}
          <span>🔒</span>
        {/if}
      </div>
    {/each}
  </section>

  <aside class="panel">
    <p>金币：—</p>
    <p>背包：—</p>
    <p>下一步将接入 Rust 权威状态。</p>
  </aside>
</main>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    background: #f4efe6;
    color: #1c1917;
    font-family: "Segoe UI", system-ui, sans-serif;
  }

  .farm {
    display: grid;
    grid-template-columns: 1fr 220px;
    gap: 1rem;
    padding: 1.25rem;
    min-height: 100vh;
    box-sizing: border-box;
  }

  header {
    grid-column: 1 / -1;
  }

  h1 {
    margin: 0 0 0.25rem;
    font-size: 1.35rem;
  }

  header p {
    margin: 0;
    opacity: 0.75;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.75rem;
    align-content: start;
  }

  .plot {
    aspect-ratio: 1;
    border-radius: 12px;
    background: #c8b48a;
    display: grid;
    place-items: center;
    font-weight: 600;
  }

  .plot.locked {
    background: #d9d2c5;
    color: #78716c;
  }

  .panel {
    background: #fffdf8;
    border: 1px solid #e7e0d4;
    border-radius: 12px;
    padding: 1rem;
    align-self: start;
  }

  .panel p {
    margin: 0 0 0.5rem;
    font-size: 0.95rem;
  }
</style>
