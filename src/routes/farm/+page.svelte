<script lang="ts">
  import { onMount } from "svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import {
    buySeed,
    clearWilted,
    cropIdFromFood,
    cropIdFromSeed,
    exportSave,
    farmSync,
    feedPet,
    foodItems,
    harvestPlot,
    importSave,
    listCrops,
    plantCrop,
    restPet,
    seedItems,
    sellItem,
    unlockPlot,
    waterPlot,
    type AchievementView,
    type CropDef,
    type GameState,
    type PlotView,
  } from "$lib/game";

  type Sheet = null | "shop" | "bag" | "more";

  let state = $state<GameState | null>(null);
  let board = $state<PlotView[]>([]);
  let crops = $state<CropDef[]>([]);
  let achievements = $state<AchievementView[]>([]);
  let selectedCrop = $state("lettuce");
  let sheet = $state<Sheet>(null);
  let toast = $state("");
  let err = $state("");
  let bootErr = $state("");
  let flashPlot = $state<number | null>(null);
  let primed = false;
  let fetchedAt = 0;
  let clock = $state(Date.now());
  const wasReady = new Set<number>();

  const cropEmoji: Record<string, string> = {
    lettuce: "🥬",
    carrot: "🥕",
    tomato: "🍅",
    strawberry: "🍓",
    apple: "🍎",
    corn: "🌽",
  };

  function cropName(id: string) {
    return crops.find((c) => c.id === id)?.name ?? id;
  }

  function seedCount(cropId: string) {
    return state?.inventory[`seed_${cropId}`] ?? 0;
  }

  function sellPriceOf(cropId: string) {
    return crops.find((c) => c.id === cropId)?.sellPrice ?? 0;
  }

  function cropArt(cropId: string | null, stage: 0 | 1 | 2 | 3, wilted = false) {
    if (wilted) return "/farm/crops/wilted.png?v=40";
    if (!cropId) return "";
    const key = stage <= 1 ? "sprout" : stage === 2 ? "grow" : "ready";
    return `/farm/crops/${cropId}-${key}.png?v=40`;
  }

  function seedArt(cropId: string) {
    return `/farm/crops/${cropId}-seed.png?v=40`;
  }

  function bar(plot: PlotView) {
    if (plot.phase === "ready") return 1;
    if ((plot.phase !== "dry" && plot.phase !== "growing") || plot.progress >= 1) {
      return plot.progress;
    }
    const need = plot.secondsLeft / Math.max(0.001, 1 - plot.progress);
    const elapsed = Math.max(0, Math.floor((clock - fetchedAt) / 1000));
    return Math.min(1, plot.progress + elapsed / need);
  }

  function remain(plot: PlotView) {
    if (plot.phase !== "dry" && plot.phase !== "growing") return plot.secondsLeft;
    const elapsed = Math.max(0, Math.floor((clock - fetchedAt) / 1000));
    return Math.max(0, plot.secondsLeft - elapsed);
  }

  function fmt(secs: number) {
    if (secs < 60) return `${secs}秒`;
    const m = Math.floor(secs / 60);
    const s = secs % 60;
    return s ? `${m}分${s}秒` : `${m}分`;
  }

  function growthStage(plot: PlotView): 0 | 1 | 2 | 3 {
    if (plot.phase === "ready") return 3;
    if (plot.phase === "wilted") return 2;
    if (plot.phase === "dry") return 1;
    if (plot.phase === "growing") {
      const p = bar(plot);
      if (p < 0.35) return 1;
      if (p < 0.75) return 2;
      return 3;
    }
    return 0;
  }

  function plotHint(plot: PlotView) {
    switch (plot.phase) {
      case "locked":
        return `开垦 ${plot.unlockCost}金`;
      case "empty":
        return "可播种";
      case "dry":
        return "要浇水";
      case "growing":
        return remain(plot) === 0 ? "快好了" : fmt(remain(plot));
      case "ready":
        return "可收获";
      case "wilted":
        return "清理";
      default:
        return "";
    }
  }

  function noteReady(plots: PlotView[]) {
    if (!primed) {
      for (const plot of plots) {
        if (plot.phase === "ready") wasReady.add(plot.index);
      }
      primed = true;
      return;
    }
    for (const plot of plots) {
      if (plot.phase === "ready" && !wasReady.has(plot.index)) {
        toast = `${plot.cropName ?? "作物"}熟了！`;
        wasReady.add(plot.index);
      }
      if (plot.phase !== "ready") wasReady.delete(plot.index);
    }
  }

  async function refresh() {
    const sync = await farmSync();
    state = sync.state;
    board = sync.plots;
    achievements = sync.achievements;
    fetchedAt = Date.now();
    clock = fetchedAt;
    noteReady(sync.plots);
    if (seedCount(selectedCrop) === 0) {
      const withSeed = crops.find((c) => (sync.state.inventory[`seed_${c.id}`] ?? 0) > 0);
      if (withSeed) selectedCrop = withSeed.id;
    }
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
      clock = Date.now();
      const due = board.some(
        (plot) => (plot.phase === "growing" || plot.phase === "dry") && remain(plot) === 0,
      );
      if (!due && Date.now() - fetchedAt < 3000) return;
      void refresh().catch((e) => {
        err = String(e);
      });
    }, 1000);
    return () => clearInterval(t);
  });

  function flash(i: number) {
    flashPlot = i;
    setTimeout(() => {
      if (flashPlot === i) flashPlot = null;
    }, 420);
  }

  function say(msg: string) {
    toast = msg;
    err = "";
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

  /** One-tap QQ-farm style: plot does the next sensible action. */
  async function onPlot(plot: PlotView) {
    flash(plot.index);
    if (!state) return;

    if (plot.phase === "locked") {
      await run(async () => {
        state = await unlockPlot(plot.index);
        say(`开垦了第 ${plot.index + 1} 块地`);
      });
      return;
    }

    if (plot.phase === "empty") {
      const have = seedCount(selectedCrop);
      await run(async () => {
        if (have <= 0) {
          const def = crops.find((c) => c.id === selectedCrop);
          if (!def) throw new Error("没有这种作物");
          if (state!.coins < def.seedPrice) {
            sheet = "shop";
            throw new Error(`金币不够买${def.name}种子`);
          }
          state = await buySeed(selectedCrop, 1);
        }
        state = await plantCrop(plot.index, selectedCrop);
        say(`种下了${cropName(selectedCrop)}，点一下浇水`);
      });
      return;
    }

    if (plot.phase === "dry") {
      await run(async () => {
        state = await waterPlot(plot.index);
        say("浇好了，等它长大");
      });
      return;
    }

    if (plot.phase === "growing") {
      say(
        remain(plot) === 0
          ? `${plot.cropName}马上就能收`
          : `${plot.cropName}还要 ${fmt(remain(plot))}`,
      );
      return;
    }

    if (plot.phase === "ready") {
      await run(async () => {
        const [next, msg] = await harvestPlot(plot.index);
        state = next;
        say(msg);
      });
      return;
    }

    if (plot.phase === "wilted") {
      await run(async () => {
        state = await clearWilted(plot.index);
        say("清理干净，可以重种");
      });
    }
  }

  async function onExport() {
    err = "";
    const path = await save({
      defaultPath: "deskpet-farm.json",
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (!path) return;
    try {
      await exportSave(path);
      say("存档已导出");
    } catch (e) {
      err = String(e);
    }
  }

  async function onImport() {
    err = "";
    const path = await open({
      multiple: false,
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (typeof path !== "string") return;
    try {
      state = await importSave(path);
      primed = false;
      wasReady.clear();
      await refresh();
      say("存档已导入");
      sheet = null;
    } catch (e) {
      err = String(e);
    }
  }

  const foods = $derived(state ? foodItems(state.inventory) : []);
  const seeds = $derived(state ? seedItems(state.inventory) : []);
</script>

<main class="farm">
  <div class="backdrop" aria-hidden="true">
    <div class="scene"></div>
  </div>

  <div class="layout">
  <header class="hud">
    <div class="brand">
      <strong>小爪农场</strong>
      {#if state}
        <span>{state.pet.name}的菜地</span>
      {/if}
    </div>
    {#if state}
      <div class="stats">
        <span class="coin" title="金币">🪙 {state.coins}</span>
        <span title="体力">⚡ {state.pet.energy}</span>
        <span title="心情">💗 {state.pet.mood}</span>
        <span title="饱腹">🍽️ {state.pet.hunger}</span>
        <button
          type="button"
          class="rest"
          onclick={() =>
            run(async () => {
              const [next, msg] = await restPet();
              state = next;
              say(msg);
            })}>休息</button
        >
      </div>
    {/if}
  </header>

  <div class="stage">
  {#if state}
    <section class="field" aria-label="菜地">
      <div class="plots-pane">
        <div class="soil-bed">
          {#each board as plot (plot.index)}
            {@const stage = growthStage(plot)}
            {@const art = cropArt(plot.cropId, stage, plot.phase === "wilted")}
            <button
              type="button"
              class="plot {plot.phase}"
              class:flash={flashPlot === plot.index}
              aria-label={`地块${plot.index + 1} ${plotHint(plot)}`}
              onclick={() => void onPlot(plot)}
            >
              <span class="dirt" aria-hidden="true"></span>

              {#if plot.phase === "locked"}
                <span class="lock">🔒</span>
                <span class="badge">{plot.unlockCost}金</span>
              {:else if art}
                <img
                  class="crop stage-{stage}"
                  class:wilt={plot.phase === "wilted"}
                  class:dry={plot.phase === "dry"}
                  class:ready={plot.phase === "ready"}
                  src={art}
                  alt=""
                  draggable="false"
                />
              {/if}

              {#if plot.phase === "dry"}
                <span class="action-badge water" aria-hidden="true"
                  ><img src="/farm/ui/water-can.png?v=40" alt="" /></span
                >
              {:else if plot.phase === "ready"}
                <span class="action-badge harvest">✨</span>
              {/if}

              {#if plot.phase === "dry" || plot.phase === "growing"}
                <span class="growbar" aria-hidden="true"
                  ><i style="width: {Math.round(bar(plot) * 100)}%"></i></span
                >
              {/if}

              <span class="hint">{plotHint(plot)}</span>
            </button>
          {/each}
        </div>
        <p class="guide">点地块就能种 / 浇 / 收 · 先选种子再点空地</p>
      </div>
    </section>
  {:else}
    <div class="boot-wrap">
      <p class="boot">唤醒农场…</p>
      {#if bootErr}
        <p class="err-inline">{bootErr}</p>
      {/if}
    </div>
  {/if}
  </div>
  </div>

  {#if state}
    <nav class="dock" aria-label="工具">
      <div class="seed-tray" role="listbox" aria-label="种子">
        {#each crops as c}
          <button
            type="button"
            role="option"
            class="seed"
            class:on={selectedCrop === c.id}
            aria-selected={selectedCrop === c.id}
            onclick={() => (selectedCrop = c.id)}
            title="{c.name} · {Math.round(c.growSecs / 60)}分钟"
          >
            <img class="ico" src={seedArt(c.id)} alt="" draggable="false" />
            <span class="nm">{c.name}</span>
            <span class="qty">×{seedCount(c.id)}</span>
          </button>
        {/each}
      </div>
      <div class="tools">
        <button type="button" class:on={sheet === "shop"} onclick={() => (sheet = sheet === "shop" ? null : "shop")}
          >商店</button
        >
        <button type="button" class:on={sheet === "bag"} onclick={() => (sheet = sheet === "bag" ? null : "bag")}
          >仓库</button
        >
        <button type="button" class:on={sheet === "more"} onclick={() => (sheet = sheet === "more" ? null : "more")}
          >更多</button
        >
      </div>
    </nav>

    {#if sheet}
      <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
      <div class="scrim" onclick={() => (sheet = null)}></div>
      <aside class="sheet" role="dialog" aria-label={sheet === "shop" ? "商店" : sheet === "bag" ? "仓库" : "更多"}>
        <header>
          <h2>{sheet === "shop" ? "种子商店" : sheet === "bag" ? "仓库" : "更多"}</h2>
          <button type="button" class="x" onclick={() => (sheet = null)}>关闭</button>
        </header>

        {#if sheet === "shop"}
          <div class="shop-grid">
            {#each crops as c}
              <button
                type="button"
                class="goods"
                onclick={() =>
                  run(async () => {
                    state = await buySeed(c.id, 1);
                    selectedCrop = c.id;
                    say(`买了${c.name}种子`);
                  })}
              >
                <img class="ico" src={seedArt(c.id)} alt="" draggable="false" />
                <strong>{c.name}</strong>
                <span>{c.seedPrice}金 · {Math.round(c.growSecs / 60)}分</span>
              </button>
            {/each}
          </div>
        {:else if sheet === "bag"}
          {#if foods.length === 0 && seeds.length === 0}
            <p class="empty">仓库空空的，去地里收点菜吧</p>
          {:else}
            <ul class="bag-list">
              {#each seeds as [id, qty]}
                <li>
                  <span>{cropEmoji[cropIdFromSeed(id)] ?? "🌱"} {cropName(cropIdFromSeed(id))}种子 ×{qty}</span>
                  <button type="button" onclick={() => (selectedCrop = cropIdFromSeed(id), (sheet = null))}
                    >选用</button
                  >
                </li>
              {/each}
              {#each foods as [id, qty]}
                <li>
                  <span
                    >{cropEmoji[cropIdFromFood(id)] ?? "🥗"} {cropName(cropIdFromFood(id))} ×{qty}</span
                  >
                  <div class="acts">
                    <button
                      type="button"
                      onclick={() =>
                        run(async () => {
                          const [next, msg] = await feedPet(id);
                          state = next;
                          say(msg);
                        })}>喂宠物</button
                    >
                    <button
                      type="button"
                      onclick={() =>
                        run(async () => {
                          state = await sellItem(id, 1);
                          say(`卖出${cropName(cropIdFromFood(id))} +${sellPriceOf(cropIdFromFood(id))}金`);
                        })}>卖 {sellPriceOf(cropIdFromFood(id))}</button
                    >
                  </div>
                </li>
              {/each}
            </ul>
          {/if}
        {:else}
          <div class="more">
            <h3>成就</h3>
            <ul class="achievements">
              {#each achievements as a}
                <li class:done={a.unlocked}>
                  <strong>{a.unlocked ? "✓" : "○"} {a.name}</strong>
                  <span>{a.description}</span>
                </li>
              {/each}
            </ul>
            <h3>存档</h3>
            <div class="row">
              <button type="button" onclick={() => void onExport()}>导出</button>
              <button type="button" onclick={() => void onImport()}>导入</button>
            </div>
            <p class="meta">收获 {state.totalHarvests} 次</p>
          </div>
        {/if}
      </aside>
    {/if}

    {#if toast || err}
      <p class="toast" class:bad={!!err} role="status">{err || toast}</p>
    {/if}
  {/if}
</main>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    height: 100%;
    overflow: hidden;
    background: #7ec850 !important;
    color: #1e2a14;
    font-family:
      "Segoe UI",
      "PingFang SC",
      "Microsoft YaHei",
      system-ui,
      sans-serif;
  }

  .farm {
    position: relative;
    box-sizing: border-box;
    width: 100%;
    height: 100vh;
    overflow: hidden;
    background: #7ec850;
  }

  .backdrop {
    position: absolute;
    inset: 0;
    z-index: 0;
    pointer-events: none;
    overflow: hidden;
  }

  .scene {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(180deg, rgba(135, 206, 250, 0.15), rgba(40, 80, 20, 0.08)),
      url("/farm/scene.png?v=40") 50% 42% / cover no-repeat;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
  }

  .layout {
    position: relative;
    z-index: 1;
    box-sizing: border-box;
    height: 100%;
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
  }

  .stage {
    position: relative;
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    padding-bottom: 0.45rem;
  }

  .hud {
    position: relative;
    z-index: 2;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem 1rem 0.4rem;
  }

  .brand {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
  }

  .brand strong {
    font-size: 1.35rem;
    letter-spacing: 0.06em;
    color: #fff8e7;
    text-shadow:
      2px 0 #3b2a14,
      -2px 0 #3b2a14,
      0 2px #3b2a14,
      0 -2px #3b2a14;
  }

  .brand span {
    font-size: 0.82rem;
    color: #f0ffe0;
    text-shadow: 0 1px 0 rgba(30, 40, 10, 0.55);
    opacity: 0.95;
  }

  .stats {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.45rem;
    padding: 0.35rem 0.55rem;
    border-radius: 4px;
    background: #c4a574;
    border: 3px solid #5a3d1e;
    box-shadow: inset 0 0 0 2px #e8d4a8;
    font-size: 0.85rem;
    font-weight: 700;
    color: #2a1a0a;
  }

  .coin {
    color: #7a4a00;
  }

  .rest {
    border: 2px solid #5a3d1e;
    border-radius: 3px;
    padding: 0.25rem 0.7rem;
    background: #e8b84a;
    color: #2a1a0a;
    font-weight: 700;
    cursor: pointer;
  }

  .field {
    position: relative;
    z-index: 1;
    flex: 1;
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0.15rem 1rem 7.2rem;
  }

  .plots-pane {
    flex: 1;
    min-height: 0;
    width: 100%;
    overflow: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.35rem;
  }

  .boot-wrap {
    flex: 1;
    display: grid;
    place-items: center;
    gap: 0.4rem;
  }

  .err-inline {
    color: #b91c1c;
    font-size: 0.85rem;
  }

  .soil-bed {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 10px;
    width: min(360px, 78vw);
    margin: 0.6rem auto 0;
    padding: 12px;
    background: rgba(90, 60, 30, 0.35);
    border: 4px solid #5a3d1e;
    box-shadow:
      inset 0 0 0 3px #c4a574,
      0 6px 0 rgba(40, 24, 8, 0.35);
    image-rendering: pixelated;
  }

  .plot {
    position: relative;
    aspect-ratio: 1;
    width: 100%;
    border: 0;
    padding: 0;
    background: transparent;
    cursor: pointer;
    overflow: visible;
    appearance: none;
    image-rendering: pixelated;
  }

  .dirt {
    position: absolute;
    inset: 6%;
    background: url("/farm/soil-mound.png?v=40") center / 100% 100% no-repeat;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
    box-shadow: 0 4px 0 rgba(40, 24, 8, 0.35);
    transition:
      filter 120ms ease,
      transform 120ms ease;
  }

  .plot.growing .dirt,
  .plot.dry .dirt,
  .plot.ready .dirt,
  .plot.wilted .dirt {
    background-image: url("/farm/soil-wet.png?v=40");
  }

  .plot.locked .dirt {
    background-image: url("/farm/soil-locked.png?v=40");
    filter: grayscale(0.4) brightness(0.9);
    opacity: 0.92;
  }

  .crop {
    position: absolute;
    left: 50%;
    bottom: 18%;
    z-index: 2;
    width: 58%;
    height: auto;
    transform: translateX(-50%);
    image-rendering: pixelated;
    image-rendering: crisp-edges;
    filter: drop-shadow(0 2px 0 rgba(40, 20, 8, 0.4));
    pointer-events: none;
    transition:
      transform 160ms ease,
      bottom 160ms ease,
      width 160ms ease;
  }

  .crop.stage-1 {
    width: 36%;
    bottom: 22%;
  }

  .crop.stage-2 {
    width: 48%;
    bottom: 20%;
  }

  .crop.stage-3,
  .crop.ready {
    width: 64%;
    bottom: 16%;
  }

  .crop.ready {
    animation: bob 1.1s ease-in-out infinite;
  }

  .crop.dry {
    filter: drop-shadow(0 2px 0 rgba(40, 20, 8, 0.4)) saturate(0.65) brightness(1.1);
  }

  .crop.wilt {
    filter: grayscale(0.85) brightness(0.88);
    transform: translateX(-50%) rotate(-8deg);
  }

  .lock {
    position: absolute;
    left: 50%;
    top: 38%;
    z-index: 2;
    transform: translate(-50%, -50%);
    font-size: 1.25rem;
  }

  .badge,
  .hint {
    position: absolute;
    z-index: 3;
    left: 50%;
    transform: translateX(-50%);
    white-space: nowrap;
    pointer-events: none;
  }

  .badge {
    top: 62%;
    padding: 0.1rem 0.42rem;
    border-radius: 2px;
    background: #2a1a0a;
    border: 2px solid #c4a574;
    color: #fef3c7;
    font-size: 0.7rem;
    font-weight: 700;
  }

  .hint {
    bottom: -6px;
    padding: 0.08rem 0.35rem;
    border-radius: 2px;
    font-size: 0.65rem;
    font-weight: 700;
    color: #fffbeb;
    background: #3d5c1a;
    border: 2px solid #1e2a0c;
    opacity: 0;
    transition: opacity 120ms ease;
  }

  .plot:hover .hint,
  .plot:focus-visible .hint,
  .plot.locked .hint,
  .plot.dry .hint,
  .plot.ready .hint,
  .plot.wilted .hint {
    opacity: 1;
  }

  .action-badge {
    position: absolute;
    top: 0;
    right: 0;
    z-index: 3;
    width: 28px;
    height: 28px;
    display: grid;
    place-items: center;
    animation: pop 1.2s ease-in-out infinite;
  }

  .action-badge img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    image-rendering: pixelated;
  }

  .growbar {
    position: absolute;
    left: 14%;
    right: 14%;
    bottom: 8%;
    z-index: 2;
    height: 6px;
    border-radius: 0;
    background: #2a1a0a;
    border: 2px solid #5a3d1e;
    overflow: hidden;
  }

  .growbar i {
    display: block;
    height: 100%;
    background: #5ecf3a;
  }

  .plot.flash .dirt,
  .plot:hover .dirt {
    filter: brightness(1.12);
    transform: translateY(-2px);
  }

  .plot.locked:hover .dirt {
    filter: grayscale(0.25) brightness(1);
    opacity: 1;
  }

  .plot:active .crop {
    transform: translateX(-50%) scale(0.92);
  }

  .guide {
    margin: 0.55rem 0 0;
    padding: 0.25rem 0.65rem;
    border-radius: 2px;
    font-size: 0.74rem;
    color: #fffbeb;
    background: #3d5c1a;
    border: 2px solid #1e2a0c;
  }

  .dock {
    position: fixed;
    left: 50%;
    bottom: 0.55rem;
    z-index: 30;
    transform: translateX(-50%);
    width: min(560px, 94%);
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.45rem 0.55rem 0.5rem;
    border-radius: 4px;
    background: #c4a574;
    border: 4px solid #5a3d1e;
    box-shadow:
      inset 0 0 0 2px #e8d4a8,
      0 6px 0 rgba(40, 24, 8, 0.28);
  }

  .seed-tray {
    display: flex;
    gap: 0.4rem;
    overflow-x: auto;
    padding-bottom: 0.15rem;
  }

  .seed {
    flex: 0 0 auto;
    display: grid;
    grid-template-columns: auto 1fr;
    grid-template-rows: auto auto;
    column-gap: 0.35rem;
    align-items: center;
    min-width: 5.4rem;
    border: 3px solid #8b6914;
    border-radius: 3px;
    padding: 0.35rem 0.5rem;
    background: #efe0c0;
    cursor: pointer;
  }

  .seed.on {
    border-color: #2a1a0a;
    background: #ffe08a;
    box-shadow: inset 0 0 0 2px #fff3c4;
  }

  .seed .ico {
    grid-row: 1 / span 2;
    width: 30px;
    height: 30px;
    object-fit: contain;
    image-rendering: pixelated;
  }

  .seed .nm {
    font-size: 0.78rem;
    font-weight: 700;
    text-align: left;
  }

  .seed .qty {
    font-size: 0.7rem;
    color: #78716c;
    text-align: left;
  }

  .tools {
    display: flex;
    gap: 0.4rem;
  }

  .tools button {
    flex: 1;
    border: 3px solid #2a4a12;
    border-radius: 3px;
    padding: 0.55rem 0.4rem;
    font-weight: 700;
    font-size: 0.9rem;
    color: #f4ffe8;
    background: #5a9e2e;
    cursor: pointer;
  }

  .tools button.on {
    background: #e8b84a;
    border-color: #5a3d1e;
    color: #2a1a0a;
  }

  .scrim {
    position: fixed;
    inset: 0;
    z-index: 8;
    background: rgba(20, 30, 10, 0.35);
  }

  .sheet {
    position: fixed;
    left: 50%;
    bottom: 0;
    z-index: 9;
    transform: translateX(-50%);
    width: min(560px, 100%);
    max-height: 62vh;
    overflow: auto;
    padding: 0.85rem 1rem 1.1rem;
    border-radius: 6px 6px 0 0;
    background: #efe0c0;
    border: 4px solid #5a3d1e;
    border-bottom: 0;
    box-shadow: 0 -8px 0 rgba(40, 30, 10, 0.2);
    animation: sheet-up 200ms ease-out;
  }

  .sheet header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.65rem;
  }

  .sheet h2 {
    margin: 0;
    font-size: 1.05rem;
  }

  .sheet h3 {
    margin: 0.85rem 0 0.35rem;
    font-size: 0.92rem;
  }

  .x {
    border: 0;
    background: #e7e5e4;
    border-radius: 999px;
    padding: 0.3rem 0.7rem;
    cursor: pointer;
    font-weight: 600;
  }

  .shop-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.5rem;
  }

  .goods {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.15rem;
    border: 3px solid #8b6914;
    border-radius: 3px;
    padding: 0.65rem 0.35rem;
    background: #fff8e6;
    cursor: pointer;
  }

  .goods .ico {
    width: 40px;
    height: 40px;
    object-fit: contain;
    image-rendering: pixelated;
  }

  .goods strong {
    font-size: 0.88rem;
  }

  .goods span {
    font-size: 0.72rem;
    color: #78716c;
  }

  .bag-list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .bag-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0;
    border-bottom: 1px solid #efe8dc;
    font-size: 0.9rem;
  }

  .acts {
    display: flex;
    gap: 0.3rem;
  }

  .bag-list button,
  .row button {
    border: 1px solid #c4b5a0;
    background: #fff;
    border-radius: 8px;
    padding: 0.3rem 0.5rem;
    cursor: pointer;
    font-size: 0.78rem;
  }

  .empty,
  .meta {
    color: #78716c;
    font-size: 0.88rem;
  }

  .achievements {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .achievements li {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    padding: 0.35rem 0;
    border-bottom: 1px solid #efe8dc;
    color: #a8a29e;
    font-size: 0.82rem;
  }

  .achievements li.done {
    color: #3f6212;
  }

  .row {
    display: flex;
    gap: 0.4rem;
  }

  .toast {
    position: fixed;
    left: 50%;
    top: 4.2rem;
    z-index: 10;
    transform: translateX(-50%);
    margin: 0;
    max-width: 80%;
    padding: 0.45rem 0.9rem;
    border-radius: 999px;
    background: rgba(28, 25, 23, 0.82);
    color: #fafaf9;
    font-size: 0.88rem;
    font-weight: 600;
    pointer-events: none;
    animation: fade-toast 220ms ease-out;
  }

  .toast.bad {
    background: rgba(153, 27, 27, 0.9);
  }

  .boot {
    position: relative;
    z-index: 2;
    margin: auto;
    font-weight: 600;
  }

  @keyframes bob {
    0%,
    100% {
      transform: translateX(-50%) translateY(0);
    }
    50% {
      transform: translateX(-50%) translateY(-5px);
    }
  }

  @keyframes pop {
    0%,
    100% {
      transform: translateY(0) scale(1);
    }
    50% {
      transform: translateY(-3px) scale(1.08);
    }
  }

  @keyframes sheet-up {
    from {
      transform: translate(-50%, 24px);
      opacity: 0.4;
    }
    to {
      transform: translate(-50%, 0);
      opacity: 1;
    }
  }

  @keyframes fade-toast {
    from {
      opacity: 0;
      transform: translate(-50%, -6px);
    }
    to {
      opacity: 1;
      transform: translate(-50%, 0);
    }
  }

  @media (max-width: 640px) {
    .shop-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .brand strong {
      font-size: 1.1rem;
    }

    .crop.stage-3,
    .crop.ready {
      font-size: 1.55rem;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .crop.ready,
    .action-badge {
      animation: none;
    }
  }
</style>
