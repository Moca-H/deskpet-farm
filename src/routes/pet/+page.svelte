<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let status = $state("idle");
  let message = $state("桌宠窗口（占位）");

  async function openFarm() {
    try {
      await invoke("open_farm_window");
      status = "farm-opened";
    } catch (err) {
      message = String(err);
      status = "error";
    }
  }
</script>

<main class="pet">
  <button
    class="sprite"
    type="button"
    title="摸摸"
    onclick={() => (message = "（以后这里会互动）")}
  >
    🐾
  </button>
  <p class="bubble">{message}</p>
  <div class="actions">
    <button type="button" onclick={openFarm}>打开农场</button>
  </div>
  <p class="meta">status: {status}</p>
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
    width: 180px;
    padding: 8px;
    text-align: center;
    color: #1c1917;
    user-select: none;
  }

  .sprite {
    width: 96px;
    height: 96px;
    border: none;
    border-radius: 24px;
    background: #f5e6c8;
    font-size: 48px;
    cursor: pointer;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.18);
  }

  .bubble {
    margin: 8px 0 0;
    font-size: 12px;
    background: rgba(255, 255, 255, 0.92);
    border-radius: 8px;
    padding: 6px 8px;
  }

  .actions {
    margin-top: 8px;
  }

  .actions button {
    border: 1px solid #c4b5a0;
    background: #fffaf0;
    border-radius: 8px;
    padding: 4px 10px;
    cursor: pointer;
    font-size: 12px;
  }

  .meta {
    margin: 6px 0 0;
    font-size: 10px;
    opacity: 0.55;
  }
</style>
