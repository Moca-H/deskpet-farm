use serde::Serialize;

use super::state::GameState;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

pub const ACHIEVEMENTS: &[AchievementDef] = &[
    AchievementDef {
        id: "first_harvest",
        name: "初次收获",
        description: "收获任意作物 1 次",
    },
    AchievementDef {
        id: "harvest_10",
        name: "小有收成",
        description: "累计收获 10 次",
    },
    AchievementDef {
        id: "harvest_50",
        name: "农场能手",
        description: "累计收获 50 次",
    },
    AchievementDef {
        id: "first_feed",
        name: "好好吃饭",
        description: "第一次喂食宠物",
    },
    AchievementDef {
        id: "pat_20",
        name: "摸摸达人",
        description: "摸摸宠物 20 次",
    },
    AchievementDef {
        id: "unlock_all",
        name: "开垦完成",
        description: "解锁全部 9 块地",
    },
    AchievementDef {
        id: "rich_300",
        name: "小金库",
        description: "同时持有 300 金币",
    },
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked: bool,
}

/// Evaluate newly unlocked achievements. Returns their display names.
pub fn sync_achievements(state: &mut GameState) -> Vec<String> {
    let unlocked_plots = state.farm.plots.iter().filter(|p| p.unlocked).count();
    let mut newly = Vec::new();

    let checks: &[(&str, bool)] = &[
        ("first_harvest", state.total_harvests >= 1),
        ("harvest_10", state.total_harvests >= 10),
        ("harvest_50", state.total_harvests >= 50),
        ("first_feed", state.total_feeds >= 1),
        ("pat_20", state.total_pats >= 20),
        ("unlock_all", unlocked_plots >= 9),
        ("rich_300", state.coins >= 300),
    ];

    for (id, ok) in checks {
        if !*ok {
            continue;
        }
        if state.achievements.iter().any(|a| a == id) {
            continue;
        }
        state.achievements.push((*id).to_string());
        if let Some(def) = ACHIEVEMENTS.iter().find(|a| a.id == *id) {
            newly.push(def.name.to_string());
        }
    }
    newly
}

pub fn list_views(state: &GameState) -> Vec<AchievementView> {
    ACHIEVEMENTS
        .iter()
        .map(|def| AchievementView {
            id: def.id.to_string(),
            name: def.name.to_string(),
            description: def.description.to_string(),
            unlocked: state.achievements.iter().any(|a| a == def.id),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::state::GameState;

    #[test]
    fn harvest_unlocks_first_achievement() {
        let mut state = GameState::default();
        state.total_harvests = 1;
        let newly = sync_achievements(&mut state);
        assert!(newly.contains(&"初次收获".to_string()));
        assert!(state.achievements.iter().any(|a| a == "first_harvest"));
        assert!(sync_achievements(&mut state).is_empty());
    }
}
