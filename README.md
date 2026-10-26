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
- 打包还需要 [Visual Studio C++ Build Tools](https://tauri.app/start/prerequisites/)

## 开发

```bash
npm install
npm run tauri dev
```

打包：

```bash
npm run tauri build
```

## 原则

- **v1 可完全离线**；AI 陪伴是后期可选能力（BYOK，可关）。
- 素材请使用自有/可授权资源，勿直接使用其它商业农场游戏的官方素材。

## License

[MIT](LICENSE)
