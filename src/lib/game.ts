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

export const getState = () => invoke<GameState>("get_state");
export const listCrops = () => invoke<CropDef[]>("list_crops");
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
export const openFarmWindow = () => invoke<void>("open_farm_window");
export const setClickThrough = (enabled: boolean) =>
  invoke<void>("set_click_through", { enabled });

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
