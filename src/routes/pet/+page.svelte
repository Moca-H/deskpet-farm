<script lang="ts">
  import { onMount, tick } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
  import PetCharacter from "$lib/PetCharacter.svelte";
  import {
    feedPet,
    foodItems,
    getState,
    openFarmWindow,
    patPet,
    restPet,
    setPetName,
    type GameState,
  } from "$lib/game";

  /** Compact fixed footprint — no dock expanding below/beside the pet. */
  const WIN = { w: 188, h: 228 };

  let state = $state<GameState | null>(null);
  let nameInput = $state("");
  let err = $state("");
  let tip = $state("");
  let panelOpen = $state(false);
  let pressed = $state(false);
  let flashHappy = $state(false);

  function moodOf(s: GameState): "idle" | "happy" | "hungry" | "tired" {
    if (flashHappy) return "happy";
    if (s.pet.hunger < 25) return "hungry";
    if (s.pet.energy < 25) return "tired";
    if (s.pet.mood > 78) return "happy";
    return "idle";
  }

  async function fitWindow() {
    await tick();
    try {
      await getCurrentWindow().setSize(new LogicalSize(WIN.w, WIN.h));
    } catch {
      /* ignore */
    }
  }

  async function togglePanel() {
    panelOpen = !panelOpen;
    if (panelOpen) tip = "";
    await fitWindow();
  }

  async function closePanel() {
    panelOpen = false;
    await fitWindow();
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
    let unlisten: (() => void) | undefined;
    void listen<{ message: string }>("crop-ready", (event) => {
      if (!panelOpen) tip = event.payload.message;
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      clearInterval(t);
      unlisten?.();
    };
  });

  let pressOrigin: { x: number; y: number } | null = null;

  function onAvatarDown(e: PointerEvent) {
    if (e.button !== 0 || panelOpen) return;
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
    if (!state || panelOpen) return;
    pressed = true;
    err = "";
    try {
      const [next, msg] = await patPet();
      state = next;
      tip = msg;
      flashHappy = true;
      setTimeout(() => (flashHappy = false), 900);
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
      panelOpen = false;
      return;
    }
    err = "";
    try {
      const [next, msg] = await feedPet(foods[0][0]);
      state = next;
      tip = msg;
      flashHappy = true;
      setTimeout(() => (flashHappy = false), 900);
      panelOpen = false;
    } catch (e) {
      tip = String(e);
      panelOpen = false;
    }
  }

  async function onRest() {
    if (!state) return;
    try {
      const [next, msg] = await restPet();
      state = next;
      tip = msg;
      panelOpen = false;
    } catch (e) {
      tip = String(e);
      panelOpen = false;
    }
  }

  async function onFarm() {
    tip = "";
    panelOpen = false;
    await openFarmWindow();
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
      panelOpen = false;
    } catch (e) {
      err = String(e);
    }
  }
</script>

<div class="root" class:panel-mode={panelOpen}>
  {#if state}
    {#if panelOpen}
      <div
        class="panel"
        role="dialog"
        aria-label="宠物操作"
        tabindex="-1"
        oncontextmenu={(e) => {
          e.preventDefault();
          void closePanel();
        }}
      >
        <p class="panel-title">{state.pet.name}</p>
        <div class="stats">
          <span>饱 {state.pet.hunger}</span>
          <span>心 {state.pet.mood}</span>
          <span>力 {state.pet.energy}</span>
        </div>

        {#if !state.onboardingDone}
          <div class="rename">
            <input
              bind:value={nameInput}
              maxlength="12"
              placeholder="起个名字"
              onkeydown={(e) => e.key === "Enter" && void onRename()}
            />
            <button type="button" class="pill" onclick={() => void onRename()}>好</button>
          </div>
        {/if}

        <button type="button" onclick={() => void onFarm()}>打开农场</button>
        <button type="button" onclick={() => void onFeed()}>喂食</button>
        <button type="button" onclick={() => void onRest()}>休息</button>
        <button type="button" class="ghost" onclick={() => void closePanel()}>返回</button>
      </div>
    {:else}
      <div class="stage">
        {#if tip}
          <p class="bubble" role="status">{tip}</p>
        {/if}

        <button
          type="button"
          class="avatar"
          aria-label={`摸摸${state.pet.name}，右键打开操作`}
          onpointerdown={onAvatarDown}
          onpointermove={onAvatarMove}
          onpointerup={onAvatarUp}
          onpointercancel={onAvatarUp}
          oncontextmenu={(e) => {
            e.preventDefault();
            void togglePanel();
          }}
        >
          <PetCharacter mood={moodOf(state)} {pressed} />
        </button>

        <div class="hud" aria-hidden="true">
          <span class="name">{state.pet.name}</span>
          <div class="bars">
            <span class="bar hunger" style="--v: {state.pet.hunger}"></span>
            <span class="bar mood" style="--v: {state.pet.mood}"></span>
            <span class="bar energy" style="--v: {state.pet.energy}"></span>
          </div>
        </div>
      </div>
    {/if}
  {:else}
    <p class="bubble">唤醒中…</p>
  {/if}

  {#if err && !panelOpen}
    <p class="err">{err}</p>
  {/if}
</div>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    background: transparent !important;
    overflow: hidden;
  }

  .root {
    box-sizing: border-box;
    width: 188px;
    height: 228px;
    color: #1f2748;
    font-family:
      "Segoe UI",
      "PingFang SC",
      "Hiragino Sans GB",
      system-ui,
      sans-serif;
    user-select: none;
  }

  .root.panel-mode {
    padding: 0;
  }

  .stage {
    position: relative;
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
  }

  .avatar {
    border: 0;
    padding: 0;
    margin: 0;
    background: transparent;
    cursor: grab;
    appearance: none;
  }

  .avatar:active {
    cursor: grabbing;
  }

  .bubble {
    position: absolute;
    top: 4px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 2;
    margin: 0;
    max-width: 156px;
    padding: 5px 9px;
    border-radius: 12px;
    font-size: 11px;
    line-height: 1.3;
    text-align: center;
    color: #2a3558;
    background: rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.75);
    pointer-events: none;
  }

  .hud {
    position: absolute;
    left: 50%;
    bottom: 6px;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 3px;
    pointer-events: none;
  }

  .name {
    font-size: 12px;
    font-weight: 650;
    color: #2a3558;
    text-shadow:
      0 1px 0 rgba(255, 255, 255, 0.9),
      0 4px 10px rgba(40, 50, 100, 0.25);
  }

  .bars {
    display: flex;
    gap: 3px;
    width: 96px;
  }

  .bar {
    flex: 1;
    height: 3px;
    border-radius: 99px;
    background: rgba(255, 255, 255, 0.5);
    overflow: hidden;
  }

  .bar::after {
    content: "";
    display: block;
    height: 100%;
    width: calc(var(--v) * 1%);
    border-radius: inherit;
  }

  .bar.hunger::after {
    background: #f59e0b;
  }

  .bar.mood::after {
    background: #ec4899;
  }

  .bar.energy::after {
    background: #3b82f6;
  }

  .panel {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 5px;
    width: 100%;
    height: 100%;
    padding: 10px 10px 8px;
    border-radius: 16px;
    background: rgba(247, 249, 255, 0.96);
    border: 1px solid rgba(255, 255, 255, 0.85);
    box-shadow: 0 10px 24px rgba(40, 50, 100, 0.18);
    outline: none;
  }

  .panel-title {
    margin: 0;
    text-align: center;
    font-size: 14px;
    font-weight: 700;
  }

  .stats {
    display: flex;
    justify-content: space-between;
    margin: 0 0 2px;
    font-size: 10px;
    color: #57534e;
  }

  .rename {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .rename input {
    flex: 1;
    min-width: 0;
    border: 0;
    outline: none;
    background: #fff;
    border-radius: 999px;
    padding: 6px 10px;
    font-size: 12px;
  }

  .pill,
  .panel button {
    border: 0;
    border-radius: 999px;
    padding: 6px 10px;
    font-size: 12px;
    font-weight: 600;
    color: #f7f9ff;
    background: linear-gradient(180deg, #8fa4e8, #5a6fc4);
    cursor: pointer;
  }

  .panel button {
    width: 100%;
  }

  .panel button.ghost {
    margin-top: auto;
    background: rgba(90, 111, 196, 0.14);
    color: #3b4b86;
  }

  .err {
    position: absolute;
    left: 8px;
    right: 8px;
    bottom: 4px;
    margin: 0;
    font-size: 10px;
    color: #b42318;
    word-break: break-all;
  }
</style>
