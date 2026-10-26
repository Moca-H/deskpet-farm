<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import {
    feedPet,
    foodItems,
    getState,
    openFarmWindow,
    patPet,
    setClickThrough,
    setPetName,
    type GameState,
  } from "$lib/game";

  let state = $state<GameState | null>(null);
  let message = $state("拖标题栏可移动；点爪印摸摸");
  let nameInput = $state("");
  let err = $state("");
  let loading = $state(true);

  async function refresh() {
    try {
      state = await getState();
      if (!nameInput && state) nameInput = state.pet.name;
      err = "";
    } catch (e) {
      err = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 5000);
    return () => clearInterval(t);
  });

  async function onPat() {
    err = "";
    try {
      const [next, msg] = await patPet();
      state = next;
      message = msg;
    } catch (e) {
      err = String(e);
    }
  }

  async function onFeed(itemId: string) {
    err = "";
    try {
      const [next, msg] = await feedPet(itemId);
      state = next;
      message = msg;
    } catch (e) {
      err = String(e);
    }
  }

  async function onRename() {
    err = "";
    try {
      state = await setPetName(nameInput);
      message = `以后就叫 ${state.pet.name}`;
    } catch (e) {
      err = String(e);
    }
  }

  async function drag() {
    try {
      await getCurrentWindow().startDragging();
    } catch {
      /* ignore */
    }
  }

  function moodEmoji(s: GameState) {
    if (s.pet.hunger < 25) return "🥺";
    if (s.pet.energy < 20) return "😴";
    if (s.pet.mood > 75) return "🥰";
    return "🐾";
  }
</script>

<main class="pet">
  <div class="titlebar" onmousedown={drag} role="presentation">deskpet · 按住拖动</div>

  {#if loading && !state}
    <p class="bubble">加载中…</p>
  {:else if state}
    <button class="sprite" type="button" title="摸摸" onclick={onPat}>
      {moodEmoji(state)}
    </button>
    <p class="name">{state.pet.name}</p>
    <div class="bars">
      <span>饱 {state.pet.hunger}</span>
      <span>心 {state.pet.mood}</span>
      <span>力 {state.pet.energy}</span>
    </div>
    <p class="bubble">{message}</p>

    {#if !state.onboardingDone}
      <div class="onboard">
        <input bind:value={nameInput} maxlength="12" placeholder="给桌宠起名" />
        <button type="button" onclick={onRename}>确认</button>
      </div>
    {/if}

    <div class="foods">
      {#each foodItems(state.inventory) as [id, qty]}
        <button type="button" onclick={() => onFeed(id)}>
          喂 {id.replace("food_", "")}×{qty}
        </button>
      {:else}
        <small>背包没有食物，去农场收获吧</small>
      {/each}
    </div>

    <div class="actions">
      <button type="button" onclick={() => openFarmWindow()}>农场</button>
      <button type="button" onclick={() => setClickThrough(true)}>穿透</button>
      <button type="button" onclick={() => setClickThrough(false)}>可点</button>
    </div>
  {:else}
    <p class="bubble">状态加载失败</p>
  {/if}

  {#if err}
    <p class="err">{err}</p>
  {/if}
</main>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    background: transparent;
    overflow: hidden;
    font-family: "Segoe UI", system-ui, sans-serif;
  }

  .pet {
    width: 204px;
    margin: 4px;
    padding: 0 8px 12px;
    text-align: center;
    color: #1c1917;
    user-select: none;
    background: rgba(255, 250, 240, 0.98);
    border: 1px solid #e7dcc8;
    border-radius: 16px;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.2);
    box-sizing: border-box;
  }

  .titlebar {
    margin: 0 -8px 8px;
    padding: 8px 10px;
    font-size: 11px;
    font-weight: 600;
    background: #efe2c9;
    border-radius: 16px 16px 0 0;
    cursor: grab;
  }

  .sprite {
    width: 96px;
    height: 96px;
    border: none;
    border-radius: 24px;
    background: #f5e6c8;
    font-size: 48px;
    cursor: pointer;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
  }

  .name {
    margin: 6px 0 0;
    font-weight: 700;
    font-size: 13px;
  }

  .bars {
    display: flex;
    justify-content: center;
    gap: 6px;
    margin-top: 4px;
    font-size: 10px;
    opacity: 0.85;
  }

  .bubble {
    margin: 8px 0 0;
    font-size: 12px;
    background: rgba(255, 255, 255, 0.92);
    border-radius: 8px;
    padding: 6px 8px;
  }

  .err {
    color: #b91c1c;
    font-size: 11px;
    margin: 8px 0 0;
    word-break: break-all;
    text-align: left;
  }

  .onboard,
  .actions,
  .foods {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    justify-content: center;
    margin-top: 8px;
  }

  input {
    width: 100px;
    border: 1px solid #c4b5a0;
    border-radius: 6px;
    padding: 2px 6px;
    font-size: 12px;
  }

  button {
    border: 1px solid #c4b5a0;
    background: #fffaf0;
    border-radius: 8px;
    padding: 3px 8px;
    cursor: pointer;
    font-size: 11px;
  }

  small {
    font-size: 10px;
    opacity: 0.7;
  }
</style>
