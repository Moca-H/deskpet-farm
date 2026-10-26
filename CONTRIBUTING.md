# Contributing

Thanks for helping with deskpet-farm.

## Dev setup

- Node.js 20+
- Rust via [rustup](https://rustup.rs/)
- Windows: WebView2 (usually preinstalled)

```bash
npm install
npm run tauri dev
```

## Workflow

1. Open an issue for larger ideas when possible.
2. Branch from `main` (`feat/...` or `fix/...`).
3. Keep PRs focused; update docs if gameplay or architecture changes.
4. Do not commit secrets, large binaries, or copyrighted third-party game art.

## Code layout

- `src/routes/pet` — pet overlay UI
- `src/routes/farm` — farm window UI
- `src-tauri/` — Rust game core, windows, tray, save
- `docs/` — architecture, gameplay, roadmap

Game rules and save mutations belong in Rust. UI only renders and calls commands.
