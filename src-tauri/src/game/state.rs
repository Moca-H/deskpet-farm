use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::catalog::unlock_cost;

pub const SAVE_VERSION: u32 = 1;
pub const OFFLINE_CAP_SECS: u64 = 8 * 3600;
pub const START_COINS: u32 = 60;
pub const START_UNLOCKED: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    pub name: String,
    /// 0 starving … 100 full
    pub hunger: i32,
    pub mood: i32,
    pub energy: i32,
    pub skin_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropInstance {
    pub crop_id: String,
    pub planted_at: i64,
    pub watered: bool,
    pub wilted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plot {
    pub unlocked: bool,
    pub crop: Option<CropInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Farm {
    pub plots: Vec<Plot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub version: u32,
    pub player_id: String,
    pub friends: Vec<String>,
    pub pet: Pet,
    pub farm: Farm,
    pub inventory: HashMap<String, u32>,
    pub coins: u32,
    pub last_tick_at: i64,
    pub total_harvests: u32,
    pub onboarding_done: bool,
}

impl Default for GameState {
    fn default() -> Self {
        let now = now_secs();
        let mut plots = Vec::with_capacity(9);
        for i in 0..9 {
            plots.push(Plot {
                unlocked: i < START_UNLOCKED,
                crop: None,
            });
        }
        let mut inventory = HashMap::new();
        inventory.insert("seed_lettuce".into(), 4);
        inventory.insert("seed_carrot".into(), 2);

        Self {
            version: SAVE_VERSION,
            player_id: Uuid::new_v4().to_string(),
            friends: vec![],
            pet: Pet {
                name: "小爪".into(),
                hunger: 70,
                mood: 70,
                energy: 80,
                skin_id: "default".into(),
            },
            farm: Farm { plots },
            inventory,
            coins: START_COINS,
            last_tick_at: now,
            total_harvests: 0,
            onboarding_done: false,
        }
    }
}

impl GameState {
    pub fn clamp_pet(&mut self) {
        self.pet.hunger = self.pet.hunger.clamp(0, 100);
        self.pet.mood = self.pet.mood.clamp(0, 100);
        self.pet.energy = self.pet.energy.clamp(0, 100);
    }

    pub fn add_item(&mut self, id: &str, qty: u32) {
        *self.inventory.entry(id.to_string()).or_insert(0) += qty;
    }

    pub fn take_item(&mut self, id: &str, qty: u32) -> Result<(), String> {
        let entry = self.inventory.get_mut(id);
        match entry {
            Some(have) if *have >= qty => {
                *have -= qty;
                if *have == 0 {
                    self.inventory.remove(id);
                }
                Ok(())
            }
            _ => Err(format!("物品不足：{id}")),
        }
    }

    pub fn unlock_price(&self, plot_index: usize) -> Result<u32, String> {
        if plot_index >= self.farm.plots.len() {
            return Err("地块不存在".into());
        }
        if self.farm.plots[plot_index].unlocked {
            return Err("已经解锁".into());
        }
        unlock_cost(plot_index).ok_or_else(|| "无法解锁".into())
    }
}

pub fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn growth_multiplier(mood: i32) -> f64 {
    if mood >= 80 {
        1.15
    } else if mood >= 50 {
        1.0
    } else if mood >= 25 {
        0.9
    } else {
        0.8
    }
}

pub fn work_efficiency(hunger: i32, energy: i32) -> f64 {
    let weak = hunger < 20 || energy < 15;
    if weak {
        0.7
    } else {
        1.0
    }
}
