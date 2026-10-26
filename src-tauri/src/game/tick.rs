use super::catalog::crop;
use super::state::{growth_multiplier, now_secs, GameState, OFFLINE_CAP_SECS};

/// Advance pet decay and crop wilt/maturity bookkeeping.
/// Returns seconds actually simulated (capped).
pub fn catch_up(state: &mut GameState) -> u64 {
    let now = now_secs();
    let raw = (now - state.last_tick_at).max(0) as u64;
    let dt = raw.min(OFFLINE_CAP_SECS);
    if dt == 0 {
        state.last_tick_at = now;
        return 0;
    }

    // Pet: hunger down, energy slowly recovers if not starving, mood drifts to mid
    let minutes = (dt / 60).max(1);
    let hunger_loss = (minutes as i32).min(40);
    state.pet.hunger -= hunger_loss;

    if state.pet.hunger >= 40 {
        state.pet.energy = (state.pet.energy + (minutes as i32 / 4).min(15)).min(100);
    } else {
        state.pet.energy -= (minutes as i32 / 6).min(10);
        state.pet.mood -= (minutes as i32 / 8).min(8);
    }

    // Crops: mark wilt if never watered past wilt window
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

    state.clamp_pet();
    state.last_tick_at = now;
    dt
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
