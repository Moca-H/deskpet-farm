# Gameplay (v1)

## Loop

1. Care for the pet (pat / feed / rest). Rest restores 30 energy, spends 4 hunger, then waits 60 seconds. Watering spends energy (8, or 12 when the pet is weak).
2. Plant, water, harvest on unlocked plots.
3. **Feed harvested veggies/fruit to the pet** (restore hunger; table may also restore mood/energy).
4. Sell surplus for coins → unlock more plots → expand farm.

## Pet stats (0–100)

| Stat | Meaning |
|------|---------|
| `hunger` | 0 = starving, 100 = full |
| `mood` | Affects crop growth slightly when high |
| `energy` | Spent on watering / tending |

Neglect should weaken efficiency, **not** wipe the save.

## Farm

- Layout: **3×3** (9 plots) on a blocky scene (Minecraft-*inspired* CC0 art; see [ART.md](ART.md)).
- Interaction: pick a seed in the dock, then **tap a plot** to plant / water / harvest (fewer menus).
- Start with **~4 unlocked**; rest locked.
- Unlock: primarily **coins**; optional light conditions (harvest count / turn in a crop) via data table.
- Crops: lettuce, carrot, tomato, strawberry, apple, corn (fast → slow), including veggie and fruit.
- Achievements unlock for harvests, feeding, pats, unlocking all plots, and holding coins.
- Light wilt rule: needs watering before mature (or times out) — avoid fully AFK farming.
- Mature crops: system tray tip + pet bubble (`crop-ready`).

## Items

Harvest → inventory. Use to:

- **Feed pet**, or
- **Sell** for coins / buy seeds

All restore amounts and prices live in data tables, not hard-coded UI.
