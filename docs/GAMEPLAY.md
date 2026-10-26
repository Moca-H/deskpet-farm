# Gameplay (v1)

## Loop

1. Care for the pet (pet / feed / rest via energy).
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

- Layout: **3×3** (9 plots).
- Start with **~4 unlocked**; rest locked.
- Unlock: primarily **coins**; optional light conditions (harvest count / turn in a crop) via data table.
- Crops: at least 3 (fast / mid / slow), including veggie and fruit flavors.
- Light wilt rule: needs watering before mature (or times out) — avoid fully AFK farming.

## Items

Harvest → inventory. Use to:

- **Feed pet**, or
- **Sell** for coins / buy seeds

All restore amounts and prices live in data tables, not hard-coded UI.
