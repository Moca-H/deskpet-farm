import { invoke } from "@tauri-apps/api/core";

export type Pet = {
  name: string;
  hunger: number;
  mood: number;
  energy: number;
  skinId: string;
};

export type CropInstance = {
  cropId: string;
  plantedAt: number;
  watered: boolean;
  wilted: boolean;
};

export type Plot = {
  unlocked: boolean;
  crop: CropInstance | null;
};

export type GameState = {
  version: number;
  playerId: string;
  friends: string[];
  pet: Pet;
  farm: { plots: Plot[] };
  inventory: Record<string, number>;
  coins: number;
  lastTickAt: number;
  totalHarvests: number;
  onboardingDone: boolean;
  restReadyAt?: number;
  totalFeeds?: number;
  totalPats?: number;
  achievements?: string[];
};

export type CropDef = {
  id: string;
  name: string;
  kind: string;
  seedPrice: number;
  sellPrice: number;
  growSecs: number;
  wiltSecs: number;
  hungerRestore: number;
  moodRestore: number;
  energyRestore: number;
};

export type AchievementView = {
  id: string;
  name: string;
  description: string;
  unlocked: boolean;
};

export const getState = () => invoke<GameState>("get_state");
export const listCrops = () => invoke<CropDef[]>("list_crops");
export const listAchievements = () => invoke<AchievementView[]>("list_achievements");
export const setPetName = (name: string) =>
  invoke<GameState>("cmd_set_pet_name", { name });
export const patPet = () => invoke<[GameState, string]>("cmd_pat");
export const feedPet = (itemId: string) =>
  invoke<[GameState, string]>("cmd_feed", { itemId });
export const plantCrop = (plotIndex: number, cropId: string) =>
  invoke<GameState>("cmd_plant", { plotIndex, cropId });
export const waterPlot = (plotIndex: number) =>
  invoke<GameState>("cmd_water", { plotIndex });
export const harvestPlot = (plotIndex: number) =>
  invoke<[GameState, string]>("cmd_harvest", { plotIndex });
export const clearWilted = (plotIndex: number) =>
  invoke<GameState>("cmd_clear_wilted", { plotIndex });
export const buySeed = (cropId: string, qty: number) =>
  invoke<GameState>("cmd_buy_seed", { cropId, qty });
export const sellItem = (itemId: string, qty: number) =>
  invoke<GameState>("cmd_sell_item", { itemId, qty });
export const unlockPlot = (plotIndex: number) =>
  invoke<GameState>("cmd_unlock_plot", { plotIndex });
export const restPet = () => invoke<[GameState, string]>("cmd_rest");

export type PlotPhase = "locked" | "empty" | "dry" | "growing" | "ready" | "wilted";

export type PlotView = {
  index: number;
  unlocked: boolean;
  phase: PlotPhase;
  cropId: string | null;
  cropName: string | null;
  secondsLeft: number;
  progress: number;
  unlockCost: number | null;
};

export type FarmSync = {
  state: GameState;
  plots: PlotView[];
  achievements: AchievementView[];
};

export const farmSync = () => invoke<FarmSync>("farm_sync");
export const exportSave = (path: string) => invoke<void>("export_save", { path });
export const importSave = (path: string) => invoke<GameState>("import_save", { path });
export const openFarmWindow = () => invoke<void>("open_farm_window");

export function foodItems(inv: Record<string, number>) {
  return Object.entries(inv).filter(([id]) => id.startsWith("food_"));
}

export function seedItems(inv: Record<string, number>) {
  return Object.entries(inv).filter(([id]) => id.startsWith("seed_"));
}

export function cropIdFromSeed(itemId: string) {
  return itemId.replace(/^seed_/, "");
}

export function cropIdFromFood(itemId: string) {
  return itemId.replace(/^food_/, "");
}

export function fmtGrow(secs: number) {
  if (secs < 60) return `${secs} 秒`;
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return s ? `${m} 分 ${s} 秒` : `${m} 分`;
}
