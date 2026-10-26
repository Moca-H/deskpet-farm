# Architecture

## Stack

- **Tauri 2** — desktop shell (WebView2 on Windows)
- **Svelte 5 + TypeScript** — pet overlay + farm UI
- **Rust** — authoritative game tick, rules, save, tray/windows

## Windows

| Window | Role |
|--------|------|
| `pet` | Transparent, always-on-top overlay; drag; light interactions |
| `farm` | Normal window; plots, inventory, shop, unlock |

Click-through for the pet window should be toggleable so the overlay does not block other apps.

## Data flow

```text
Svelte UI  --invoke-->  Tauri commands  -->  GameDomain (Rust)
                                              |-- SavePort (JSON)
                                              |-- CompanionPort (stub → later AI)
                                              \-- SocialPort (stub → later friends)
```

- **v1**: only `SavePort` is real; companions/social are stubs.
- UI never mutates economy/stats directly.

## Save

- Location: app data dir JSON file
- Fields include `version`, `playerId`, `friends: []`, `pet`, `farm`, `inventory`, `lastTickAt`
- Migrations run on load when `version` bumps

## Offline catch-up

On launch, advance simulation from `lastTickAt` with a **soft cap** (e.g. 8–12 hours) so long AFK cannot break the economy.
