<script lang="ts">
  import { onMount } from "svelte";
  import {
    buySeed,
    clearWilted,
    cropIdFromSeed,
    getState,
    harvestPlot,
    listCrops,
    plantCrop,
    sellItem,
    seedItems,
    foodItems,
    unlockPlot,
    waterPlot,
    type CropDef,
    type GameState,
  } from "$lib/game";

  const UNLOCK: Record<number, number> = { 4: 40, 5: 80, 6: 140, 7: 220, 8: 320 };

  let state = $state<GameState | null>(null);
  let crops = $state<CropDef[]>([]);
  let selectedCrop = $state("lettuce");
  let selectedPlot = $state(0);
  let log = $state("欢迎来到农场");
  let err = $state("");
  let bootErr = $state("");

  async function refresh() {
    state = await getState();
  }

  onMount(() => {
    void (async () => {
      try {
        crops = await listCrops();
        if (crops[0]) selectedCrop = crops[0].id;
        await refresh();
      } catch (e) {
        bootErr = String(e);
      }
    })();
    const t = setInterval(() => {
      void refresh().catch((e) => {
        err = String(e);
      });
    }, 3000);
    return () => clearInterval(t);
  });

  function cropName(id: string) {
    return crops.find((c) => c.id === id)?.name ?? id;
  }

  function plotLabel(i: number, s: GameState) {
    const p = s.farm.plots[i];
    if (!p.unlocked) return `🔒 ${UNLOCK[i] ?? "?"}金`;
    if (!p.crop) return "空地";
    if (p.crop.wilted) return "枯萎";
    if (!p.crop.watered) return `${cropName(p.crop.cropId)}·待浇水`;
    return `${cropName(p.crop.cropId)}·生长中`;
  }

  async function run(fn: () => Promise<void>) {
    err = "";
    try {
      await fn();
      await refresh();
    } catch (e) {
      err = String(e);
    }
  }
</script>

<main class="farm">
  <header>
    <h1>deskpet farm</h1>
    {#if state}
      <p>
        {state.pet.name} · 金币 {state.coins} · 收获 {state.totalHarvests} 次 · 饱
        {state.pet.hunger}/心{state.pet.mood}/力{state.pet.energy}
      </p>
    {/if}
  </header>

  {#if state}
    <section class="grid" aria-label="plots">
      {#each state.farm.plots as plot, i}
        <button
          type="button"
          class={["plot", !plot.unlocked && "locked", selectedPlot === i && "selected"]
            .filter(Boolean)
            .join(" ")}
          onclick={() => (selectedPlot = i)}
        >
          {plotLabel(i, state)}
        </button>
      {/each}
    </section>

    <aside class="panel">
      <h2>地块 #{selectedPlot + 1}</h2>
      <div class="row">
        <select bind:value={selectedCrop}>
          {#each crops as c}
            <option value={c.id}>{c.name}（种{c.seedPrice}金）</option>
          {/each}
        </select>
      </div>
      <div class="row">
        <button
          type="button"
          onclick={() =>
            run(async () => {
              state = await plantCrop(selectedPlot, selectedCrop);
              log = `已种植 ${cropName(selectedCrop)}`;
            })}>种植</button
        >
        <button
          type="button"
          onclick={() =>
            run(async () => {
              state = await waterPlot(selectedPlot);
              log = "浇水完成";
            })}>浇水</button
        >
        <button
          type="button"
          onclick={() =>
            run(async () => {
              const [next, msg] = await harvestPlot(selectedPlot);
              state = next;
              log = msg;
            })}>收获</button
        >
        <button
          type="button"
          onclick={() =>
            run(async () => {
              state = await clearWilted(selectedPlot);
              log = "已清理枯萎";
            })}>清理</button
        >
        <button
          type="button"
          onclick={() =>
            run(async () => {
              state = await unlockPlot(selectedPlot);
              log = `解锁地块 #${selectedPlot + 1}`;
            })}>解锁</button
        >
      </div>

      <h2>商店</h2>
      <div class="row">
        {#each crops as c}
          <button
            type="button"
            onclick={() =>
              run(async () => {
                state = await buySeed(c.id, 1);
                log = `买了 ${c.name} 种子`;
              })}>买{c.name}</button
          >
        {/each}
      </div>

      <h2>背包</h2>
      <ul>
        {#each seedItems(state.inventory) as [id, qty]}
          <li>种子 {cropName(cropIdFromSeed(id))} ×{qty}</li>
        {/each}
        {#each foodItems(state.inventory) as [id, qty]}
          <li>
            食物 {id.replace("food_", "")} ×{qty}
            <button
              type="button"
              onclick={() =>
                run(async () => {
                  state = await sellItem(id, 1);
                  log = "卖出 1 份";
                })}>卖1</button
            >
          </li>
        {/each}
      </ul>

      <p class="log">{log}</p>
      {#if err}
        <p class="err">{err}</p>
      {/if}
    </aside>
  {:else}
    <p>加载中…</p>
    {#if bootErr}
      <p class="err">{bootErr}</p>
    {/if}
  {/if}
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
    grid-template-columns: 1fr 280px;
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

  header p,
  .log {
    margin: 0;
    opacity: 0.8;
  }

  h2 {
    margin: 1rem 0 0.4rem;
    font-size: 1rem;
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
    border: 2px solid transparent;
    font-weight: 600;
    cursor: pointer;
  }

  .plot.locked {
    background: #d9d2c5;
    color: #78716c;
  }

  .plot.selected {
    border-color: #a16207;
  }

  .panel {
    background: #fffdf8;
    border: 1px solid #e7e0d4;
    border-radius: 12px;
    padding: 1rem;
    align-self: start;
  }

  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 0.5rem;
  }

  button,
  select {
    border: 1px solid #c4b5a0;
    background: #fffaf0;
    border-radius: 8px;
    padding: 0.35rem 0.55rem;
    cursor: pointer;
    font-size: 0.85rem;
  }

  ul {
    margin: 0;
    padding-left: 1.1rem;
    font-size: 0.9rem;
  }

  li button {
    margin-left: 0.35rem;
  }

  .err {
    color: #b91c1c;
  }
</style>
