use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropDef {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub seed_price: u32,
    pub sell_price: u32,
    pub grow_secs: u64,
    pub wilt_secs: u64,
    pub hunger_restore: i32,
    pub mood_restore: i32,
    pub energy_restore: i32,
}

pub const CROPS: &[CropDef] = &[
    CropDef {
        id: "lettuce",
        name: "生菜",
        kind: "veggie",
        seed_price: 5,
        sell_price: 8,
        grow_secs: 60,
        wilt_secs: 180,
        hunger_restore: 18,
        mood_restore: 2,
        energy_restore: 0,
    },
    CropDef {
        id: "carrot",
        name: "胡萝卜",
        kind: "veggie",
        seed_price: 12,
        sell_price: 20,
        grow_secs: 180,
        wilt_secs: 420,
        hunger_restore: 28,
        mood_restore: 4,
        energy_restore: 2,
    },
    CropDef {
        id: "apple",
        name: "苹果",
        kind: "fruit",
        seed_price: 25,
        sell_price: 40,
        grow_secs: 420,
        wilt_secs: 900,
        hunger_restore: 35,
        mood_restore: 10,
        energy_restore: 5,
    },
    CropDef {
        id: "tomato",
        name: "番茄",
        kind: "veggie",
        seed_price: 18,
        sell_price: 28,
        grow_secs: 240,
        wilt_secs: 540,
        hunger_restore: 24,
        mood_restore: 6,
        energy_restore: 3,
    },
    CropDef {
        id: "strawberry",
        name: "草莓",
        kind: "fruit",
        seed_price: 32,
        sell_price: 52,
        grow_secs: 360,
        wilt_secs: 780,
        hunger_restore: 22,
        mood_restore: 14,
        energy_restore: 4,
    },
    CropDef {
        id: "corn",
        name: "玉米",
        kind: "veggie",
        seed_price: 40,
        sell_price: 65,
        grow_secs: 600,
        wilt_secs: 1200,
        hunger_restore: 40,
        mood_restore: 6,
        energy_restore: 10,
    },
];

/// Unlock coin cost by plot index. First 4 plots free.
pub fn unlock_cost(plot_index: usize) -> Option<u32> {
    match plot_index {
        0..=3 => None,
        4 => Some(40),
        5 => Some(80),
        6 => Some(140),
        7 => Some(220),
        8 => Some(320),
        _ => None,
    }
}

pub fn crop(id: &str) -> Option<&'static CropDef> {
    CROPS.iter().find(|c| c.id == id)
}

pub fn seed_item_id(crop_id: &str) -> String {
    format!("seed_{crop_id}")
}

pub fn harvest_item_id(crop_id: &str) -> String {
    format!("food_{crop_id}")
}
