<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  onMount(() => {
    const path = window.location.pathname;
    if (path === "/" || path === "") {
      void goto("/pet");
    }
  });
</script>

<p>Loading deskpet-farm…</p>
