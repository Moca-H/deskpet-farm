use super::catalog::{crop, unlock_cost};
use super::state::{growth_multiplier, now_secs, GameState, OFFLINE_CAP_SECS};

/// Advance pet decay and crop wilt/maturity bookkeeping.
/// Returns seconds actually simulated (capped).
pub fn catch_up(state: &mut GameState) -> u64 {
    let now = now_secs();
    let raw = (now - state.last_tick_at).max(0) as u64;
    let dt = raw.min(OFFLINE_CAP_SECS);
    // Whole minutes only. A farm poll every few seconds must not count as a minute.
    let minutes = (dt / 60) as i32;

    if minutes > 0 {
        let hunger_loss = minutes.min(40);
        state.pet.hunger -= hunger_loss;

        if state.pet.hunger >= 40 {
            state.pet.energy = (state.pet.energy + (minutes / 4).min(15)).min(100);
        } else {
            state.pet.energy -= (minutes / 6).min(10);
            state.pet.mood -= (minutes / 8).min(8);
        }
        state.clamp_pet();
    }

    // Crops: mark wilt if never watered past wilt window. Uses wall clock, not the minute bucket.
    let mood_mul = growth_multiplier(state.pet.mood);
    for plot in &mut state.farm.plots {
        if let Some(c) = plot.crop.as_mut() {
            if c.wilted {
                continue;
            }
            let Some(def) = crop(&c.crop_id) else {
                continue;
            };
            let age = (now - c.planted_at).max(0) as u64;
            let wilt_at = ((def.wilt_secs as f64) / mood_mul) as u64;
            if !c.watered && age >= wilt_at {
                c.wilted = true;
            }
        }
    }

    if raw > OFFLINE_CAP_SECS {
        state.last_tick_at = now;
    } else if minutes > 0 {
        state.last_tick_at += i64::from(minutes) * 60;
    }
    (minutes.max(0) as u64) * 60
}

pub fn needed_grow_secs(state: &GameState, crop_id: &str) -> Option<u64> {
    let def = crop(crop_id)?;
    let mul = growth_multiplier(state.pet.mood);
    Some(((def.grow_secs as f64) / mul).ceil() as u64)
}

pub fn is_mature(state: &GameState, planted_at: i64, crop_id: &str, watered: bool, wilted: bool) -> bool {
    if wilted || !watered {
        return false;
    }
    let Some(need) = needed_grow_secs(state, crop_id) else {
        return false;
    };
    let age = (now_secs() - planted_at).max(0) as u64;
    age >= need
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlotView {
    pub index: usize,
    pub unlocked: bool,
    /// locked, empty, dry, growing, ready, wilted
    pub phase: &'static str,
    pub crop_id: Option<String>,
    pub crop_name: Option<String>,
    pub seconds_left: u64,
    /// 0..1 toward the next event (wilt or harvest).
    pub progress: f32,
    pub unlock_cost: Option<u32>,
}

pub fn plot_views(state: &GameState) -> Vec<PlotView> {
    let now = now_secs();
    state
        .farm
        .plots
        .iter()
        .enumerate()
        .map(|(index, plot)| {
            let locked_cost = if plot.unlocked {
                None
            } else {
                unlock_cost(index)
            };
            if !plot.unlocked {
                return PlotView {
                    index,
                    unlocked: false,
                    phase: "locked",
                    crop_id: None,
                    crop_name: None,
                    seconds_left: 0,
                    progress: 0.0,
                    unlock_cost: locked_cost,
                };
            }
            let Some(inst) = &plot.crop else {
                return PlotView {
                    index,
                    unlocked: true,
                    phase: "empty",
                    crop_id: None,
                    crop_name: None,
                    seconds_left: 0,
                    progress: 0.0,
                    unlock_cost: None,
                };
            };
            let crop_id = Some(inst.crop_id.clone());
            let Some(def) = crop(&inst.crop_id) else {
                return PlotView {
                    index,
                    unlocked: true,
                    phase: "wilted",
                    crop_id,
                    crop_name: None,
                    seconds_left: 0,
                    progress: 1.0,
                    unlock_cost: None,
                };
            };
            let crop_name = Some(def.name.to_string());
            if inst.wilted {
                return PlotView {
                    index,
                    unlocked: true,
                    phase: "wilted",
                    crop_id,
                    crop_name,
                    seconds_left: 0,
                    progress: 1.0,
                    unlock_cost: None,
                };
            }
            let age = (now - inst.planted_at).max(0) as u64;
            if !inst.watered {
                let wilt_at = (((def.wilt_secs as f64) / growth_multiplier(state.pet.mood)).ceil()
                    as u64)
                    .max(1);
                return PlotView {
                    index,
                    unlocked: true,
                    phase: "dry",
                    crop_id,
                    crop_name,
                    seconds_left: wilt_at.saturating_sub(age),
                    progress: (age as f32 / wilt_at as f32).clamp(0.0, 1.0),
                    unlock_cost: None,
                };
            }
            let need = needed_grow_secs(state, &inst.crop_id).unwrap_or(1).max(1);
            if age >= need {
                return PlotView {
                    index,
                    unlocked: true,
                    phase: "ready",
                    crop_id,
                    crop_name,
                    seconds_left: 0,
                    progress: 1.0,
                    unlock_cost: None,
                };
            }
            PlotView {
                index,
                unlocked: true,
                phase: "growing",
                crop_id,
                crop_name,
                seconds_left: need - age,
                progress: age as f32 / need as f32,
                unlock_cost: None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::state::CropInstance;

    fn lettuce_plot(age_secs: i64, watered: bool) -> GameState {
        let mut state = GameState::default();
        state.pet.mood = 70;
        state.farm.plots[0].crop = Some(CropInstance {
            crop_id: "lettuce".into(),
            planted_at: now_secs() - age_secs,
            watered,
            wilted: false,
        });
        state
    }

    #[test]
    fn partial_minute_does_not_drain_hunger() {
        let mut state = GameState::default();
        state.pet.hunger = 70;
        state.last_tick_at = now_secs() - 3;
        assert_eq!(catch_up(&mut state), 0);
        assert_eq!(state.pet.hunger, 70);
    }

    #[test]
    fn one_minute_drains_one_hunger() {
        let mut state = GameState::default();
        state.pet.hunger = 70;
        state.last_tick_at = now_secs() - 90;
        assert_eq!(catch_up(&mut state), 60);
        assert_eq!(state.pet.hunger, 69);
    }

    #[test]
    fn dry_plot_counts_down_to_wilt() {
        let view = plot_views(&lettuce_plot(10, false));
        assert_eq!(view[0].phase, "dry");
        assert!(view[0].seconds_left > 60);
        assert!(view[0].progress < 0.5);
    }

    #[test]
    fn watered_plot_becomes_ready() {
        let view = plot_views(&lettuce_plot(120, true));
        assert_eq!(view[0].phase, "ready");
        assert_eq!(view[0].crop_name.as_deref(), Some("生菜"));
    }
}
