<script lang="ts">
  import { onMount, tick } from "svelte";
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
  import PetCharacter from "$lib/PetCharacter.svelte";
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
  let nameInput = $state("");
  let err = $state("");
  let tip = $state("");
  let menuOpen = $state(false);
  let pressed = $state(false);
  let flashHappy = $state(false);
  let rootEl = $state<HTMLElement | null>(null);

  function moodOf(s: GameState): "idle" | "happy" | "hungry" | "tired" {
    if (flashHappy) return "happy";
    if (s.pet.hunger < 25) return "hungry";
    if (s.pet.energy < 25) return "tired";
    if (s.pet.mood > 78) return "happy";
    return "idle";
  }

  async function fitWindow() {
    await tick();
    if (!rootEl) return;
    try {
      const r = rootEl.getBoundingClientRect();
      await getCurrentWindow().setSize(
        new LogicalSize(Math.ceil(r.width + 2), Math.ceil(r.height + 2)),
      );
    } catch {
      /* ignore */
    }
  }

  async function refresh() {
    try {
      state = await getState();
      if (!nameInput && state) nameInput = state.pet.name;
      err = "";
      await fitWindow();
    } catch (e) {
      err = String(e);
    }
  }

  onMount(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 5000);
    return () => clearInterval(t);
  });

  let pressOrigin: { x: number; y: number } | null = null;

  function onAvatarDown(e: PointerEvent) {
    if (e.button !== 0 || menuOpen) return;
    pressOrigin = { x: e.clientX, y: e.clientY };
    pressed = true;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
  }

  async function onAvatarMove(e: PointerEvent) {
    if (!pressOrigin) return;
    const dist = Math.hypot(e.clientX - pressOrigin.x, e.clientY - pressOrigin.y);
    if (dist < 7) return;
    pressOrigin = null;
    try {
      await getCurrentWindow().startDragging();
    } catch {
      /* ignore */
    }
  }

  function onAvatarUp() {
    const wasClick = pressOrigin != null;
    pressOrigin = null;
    pressed = false;
    if (wasClick) void onPat();
  }

  async function onPat() {
    if (!state) return;
    pressed = true;
    err = "";
    try {
      const [next, msg] = await patPet();
      state = next;
      tip = msg;
      flashHappy = true;
      setTimeout(() => (flashHappy = false), 900);
      await fitWindow();
    } catch (e) {
      err = String(e);
    } finally {
      setTimeout(() => (pressed = false), 120);
    }
  }

  async function onFeed() {
    if (!state) return;
    const foods = foodItems(state.inventory);
    if (!foods.length) {
      tip = "还没有收获的食物";
      return;
    }
    err = "";
    try {
      const [next, msg] = await feedPet(foods[0][0]);
      state = next;
      tip = msg;
      flashHappy = true;
      setTimeout(() => (flashHappy = false), 900);
      menuOpen = false;
      await fitWindow();
    } catch (e) {
      err = String(e);
    }
  }

  async function onRename() {
    err = "";
    const name = nameInput.trim();
    if (!name) {
      err = "名字不能为空";
      return;
    }
    try {
      state = await setPetName(name);
      tip = `叫我 ${state.pet.name} 吧`;
      await fitWindow();
    } catch (e) {
      err = String(e);
    }
  }
</script>

<div class="root" bind:this={rootEl}>
  {#if state}
    <button
      type="button"
      class="avatar"
      aria-label={`摸摸${state.pet.name}`}
      onpointerdown={onAvatarDown}
      onpointermove={onAvatarMove}
      onpointerup={onAvatarUp}
      onpointercancel={onAvatarUp}
      oncontextmenu={(e) => {
        e.preventDefault();
        menuOpen = !menuOpen;
        void fitWindow();
      }}
    >
      <PetCharacter mood={moodOf(state)} {pressed} />
    </button>

    <p class="nametag">{state.pet.name}</p>

    {#if tip}
      <p class="tip" role="status">{tip}</p>
    {/if}

    {#if !state.onboardingDone}
      <div class="glass rename">
        <input
          bind:value={nameInput}
          maxlength="12"
          placeholder="给你的伙伴起名"
          onkeydown={(e) => e.key === "Enter" && void onRename()}
        />
        <button type="button" class="pill" onclick={() => void onRename()}>好</button>
      </div>
    {/if}

    {#if menuOpen}
      <div class="glass menu" role="menu">
        <button type="button" role="menuitem" onclick={() => void openFarmWindow()}>农场</button>
        <button type="button" role="menuitem" onclick={() => void onFeed()}>喂食</button>
        <button
          type="button"
          role="menuitem"
          onclick={() => {
            void setClickThrough(true);
            tip = "已穿透，托盘可恢复点击";
            menuOpen = false;
          }}>穿透</button
        >
        <button
          type="button"
          role="menuitem"
          onclick={() => {
            menuOpen = false;
            void fitWindow();
          }}>收起</button
        >
      </div>
      <p class="hint">右键开关菜单 · 左键摸摸 · 拖拽移动</p>
    {:else}
      <p class="hint quiet">右键菜单</p>
    {/if}
  {:else}
    <p class="tip">唤醒中…</p>
  {/if}

  {#if err}
    <p class="err">{err}</p>
  {/if}
</div>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent !important;
    overflow: hidden;
  }

  .root {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: max-content;
    padding: 6px 8px 8px;
    color: #1f2748;
    font-family:
      "Segoe UI",
      "PingFang SC",
      "Hiragino Sans GB",
      system-ui,
      sans-serif;
    user-select: none;
  }

  .avatar {
    border: 0;
    padding: 0;
    background: transparent;
    cursor: grab;
    appearance: none;
  }

  .avatar:active {
    cursor: grabbing;
  }

  .nametag {
    margin: 2px 0 0;
    font-size: 13px;
    font-weight: 650;
    letter-spacing: 0.02em;
    color: #2a3558;
    text-shadow:
      0 1px 0 rgba(255, 255, 255, 0.85),
      0 6px 14px rgba(40, 50, 100, 0.2);
  }

  .tip {
    margin: 6px 0 0;
    max-width: 150px;
    padding: 5px 9px;
    border-radius: 999px;
    font-size: 11px;
    line-height: 1.35;
    text-align: center;
    color: #2a3558;
    background: rgba(255, 255, 255, 0.55);
    backdrop-filter: blur(14px) saturate(160%);
    -webkit-backdrop-filter: blur(14px) saturate(160%);
    border: 1px solid rgba(255, 255, 255, 0.55);
    box-shadow: 0 8px 20px rgba(40, 50, 100, 0.12);
  }

  .glass {
    margin-top: 8px;
    padding: 6px;
    border-radius: 14px;
    background: rgba(247, 249, 255, 0.62);
    backdrop-filter: blur(18px) saturate(170%);
    -webkit-backdrop-filter: blur(18px) saturate(170%);
    border: 1px solid rgba(255, 255, 255, 0.65);
    box-shadow: 0 10px 24px rgba(40, 50, 100, 0.14);
  }

  .rename {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .rename input {
    width: 108px;
    border: 0;
    outline: none;
    background: rgba(255, 255, 255, 0.7);
    border-radius: 999px;
    padding: 5px 10px;
    font-size: 12px;
    color: #1f2748;
  }

  .pill,
  .menu button {
    border: 0;
    border-radius: 999px;
    padding: 5px 10px;
    font-size: 12px;
    font-weight: 600;
    color: #f7f9ff;
    background: linear-gradient(180deg, #8fa4e8, #5a6fc4);
    cursor: pointer;
  }

  .pill:active,
  .menu button:active {
    transform: scale(0.97);
  }

  .menu {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    justify-content: center;
    max-width: 168px;
  }

  .hint {
    margin: 6px 0 0;
    font-size: 10px;
    color: rgba(42, 53, 88, 0.72);
  }

  .hint.quiet {
    opacity: 0.55;
  }

  .err {
    margin: 6px 0 0;
    max-width: 160px;
    font-size: 10px;
    color: #b42318;
    word-break: break-all;
  }

  @media (prefers-reduced-transparency: reduce) {
    .tip,
    .glass {
      background: #f7f9ff;
      backdrop-filter: none;
    }
  }
</style>
