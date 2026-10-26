# 美术与素材说明

本仓库**不包含**官方 Minecraft / Mojang 贴图，也不使用需付费或限制再分发的资源包。

## 桌宠

| 资源 | 许可 | 说明 |
|------|------|------|
| `static/pets/pixel/*.png` | 与本仓库同 MIT | 为本项目绘制的橘猫像素表情帧（idle / happy / hungry / tired） |

详见 `static/pets/CREDITS.txt`。

## 农场（当前：仿 MC 方块风）

打包脚本：`python scripts/pack-farm-art.py`（源素材在本地 `tmp-assets/`，已 gitignore；产物进 `static/farm/`）。

| 来源 | 许可 | 用途 |
|------|------|------|
| [Kenney Voxel Pack](https://kenney.nl/assets/voxel-pack) | CC0 | 草地/泥土方块、栅栏、天空与装饰、部分 UI |
| [josehzz Farming crops 16x16](https://opengameart.org/content/farming-crops-16x16) | CC0 | 田间生长阶段、种子/收获图标 |
| [ChikenwingJJA Simple Farm Tiles](https://opengameart.org/content/simple-farm-tiles) | CC0 | 浇水壶等小 UI |

完整列表见 `static/farm/CREDITS.txt`。

## 交互与布局约定

- 农场窗口固定约 **1024×780**；地块 **3×3**，点地块完成种 / 浇 / 收（先选种子再点空地）。
- 视觉方向：方块像素风场景 + 方形耕地网格（便于与仿 MC 素材统一）。
- 换皮：替换 `static/farm` 对应 PNG 后 bump 查询参数（如 `?v=`），或改 `pack-farm-art.py` 再打包。
