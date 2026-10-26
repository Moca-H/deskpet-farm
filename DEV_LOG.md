# 开发日志

## [2026-10-26 16:55] - 功能迭代 + 农场视觉改版

| 项目 | 内容 |
|------|------|
| 修改描述 | ① 作物扩展（番茄/草莓/玉米）、成就、成熟托盘提示与桌宠气泡、宠物 HUD。② 农场改为 QQ 农场式「点地块」交互与场景化 UI。③ 视觉统一为仿 Minecraft 方块风（仅 CC0，无官方 MC 贴图）。④ 桌宠换为 `static/pets/pixel` 帧。 |
| 影响范围 | Rust 游戏规则/成就/tick；Tauri 窗口与通知；农场页布局与资源打包；桌宠面板与素材 |
| 问题原因（如适用） | 早期农场菜单点点点、素材混搭（Tiny + Iso）违和；用户希望更接近 MC 观感 |
| 涉及文件 | 见下方「涉及文件」 |
| 备注 | 截图脚本 `scripts/shot-farm.ps1`（CopyFromScreen）；素材重建 `python scripts/pack-farm-art.py`。Windows installer Release 仍待做。 |

### 玩法 / 后端

- 新作物：`tomato` / `strawberry` / `corn`（`catalog.rs`）
- 成就同步：`achievements.rs`，在摸头/喂食/收获/开垦等路径提示
- 成熟提醒：托盘 / 事件气泡（`lib.rs` + pet 监听 `crop-ready`）
- 存档导入导出：dialog 插件 + farm「更多」面板

### 农场 UI

- 场景化背景 + 3×3 地块；一点完成种/浇/收
- 底部种子托盘 + 商店/仓库/更多
- 仿 MC：Voxel 草地泥土 + 16x16 作物精灵；方形网格布局

### 涉及文件

`src-tauri/src/game/{achievements,catalog,rules,state,tick,mod}.rs`, `src-tauri/src/lib.rs`, `src-tauri/permissions/game.toml`, `src-tauri/capabilities/default.json`, `src-tauri/tauri.conf.json`, `src/lib/game.ts`, `src/lib/PetCharacter.svelte`, `src/routes/farm/+page.svelte`, `src/routes/pet/+page.svelte`, `scripts/pack-farm-art.py`, `scripts/shot-farm.ps1`, `static/farm/**`, `static/pets/pixel/**`, `docs/{ART,GAMEPLAY,ROADMAP}.md`, `README.md`, `DEV_LOG.md`

---

## [2026-10-26 16:50] - 视觉改版（过程记录）

| 项目 | 内容 |
|------|------|
| 修改描述 | 农场视觉改为 Minecraft 风格的方块像素风：统一 CC0 素材包、方形地块网格、木质 HUD |
| 影响范围 | 农场场景/泥土/作物/UI 资源、`pack-farm-art.py`、`farm/+page.svelte` |
| 问题原因（如适用） | 先前 Tiny Farm + Iso 混搭不协调；用户希望换成 MC 观感 |
| 涉及文件 | `scripts/pack-farm-art.py`, `static/farm/**`, `src/routes/farm/+page.svelte` |
| 备注 | 未使用官方 Minecraft/Mojang 贴图；Kenney Voxel Pack、josehzz Farming crops 16x16、ChikenwingJJA Simple Farm Tiles（CC0） |
