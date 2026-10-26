# deskpet-farm

开源 **桌面宠物 + 迷你农场**（Windows 优先）。

养成一只可置顶的桌宠，在农场里种菜/水果；**收获可以喂宠物**（恢复饥饿等），多余收成可卖金币解锁更多地块。

| 层 | 技术 |
|----|------|
| 壳 | [Tauri 2](https://tauri.app/) |
| UI | Svelte 5 + TypeScript（SvelteKit） |
| 游戏核心 | Rust（tick / 规则 / 存档） |

## 状态

🚧 早期开发中。详见 [docs/ROADMAP.md](docs/ROADMAP.md)。

## 文档

- [架构](docs/ARCHITECTURE.md)
- [玩法与数值](docs/GAMEPLAY.md)
- [路线图](docs/ROADMAP.md)
- [贡献](CONTRIBUTING.md)

## 前置

- Node.js 20+
- [Rust / rustup](https://www.rust-lang.org/tools/install)
- Windows WebView2（一般已自带）
- **MSVC `link.exe`**：安装 [Visual Studio 2022 Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)，勾选「使用 C++ 的桌面开发」  
  （Tauri 在 Windows 上依赖它；详见 [prerequisites](https://tauri.app/start/prerequisites/)）

## 开发

```bash
npm install
npm run tauri dev
```

`npm run tauri` 已通过 `scripts/run-tauri.mjs` 自动把 `%USERPROFILE%\.cargo\bin` 加进 PATH。若仍报找不到 cargo，关掉终端重开，或运行 `.\dev.cmd`。

启动后桌宠**默认可点击**；需要不挡鼠标时再用托盘「点击穿透」。

打包：

```bash
npm run tauri build
```

## 原则

- **v1 可完全离线**；AI 陪伴是后期可选能力（BYOK，可关）。
- 素材请使用自有/可授权资源，勿直接使用其它商业农场游戏的官方素材。

## License

[MIT](LICENSE)
