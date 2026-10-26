use super::catalog::{crop, harvest_item_id, seed_item_id};
use super::state::{now_secs, work_efficiency, CropInstance, GameState};
use super::tick::{catch_up, is_mature};

pub fn refresh(state: &mut GameState) {
    catch_up(state);
}

pub fn set_pet_name(state: &mut GameState, name: String) -> Result<(), String> {
    let name = name.trim().to_string();
    if name.is_empty() || name.chars().count() > 12 {
        return Err("名字需要 1～12 个字".into());
    }
    state.pet.name = name;
    state.onboarding_done = true;
    Ok(())
}

pub fn pat(state: &mut GameState) -> Result<String, String> {
    refresh(state);
    state.pet.mood = (state.pet.mood + 6).min(100);
    state.pet.energy = (state.pet.energy - 1).max(0);
    state.clamp_pet();
    Ok(format!("{} 很开心被摸摸", state.pet.name))
}

pub fn feed(state: &mut GameState, item_id: &str) -> Result<String, String> {
    refresh(state);
    if !item_id.starts_with("food_") {
        return Err("只能喂食物".into());
    }
    let crop_id = item_id.trim_start_matches("food_");
    let def = crop(crop_id).ok_or_else(|| "未知食物".to_string())?;
    state.take_item(item_id, 1)?;
    state.pet.hunger = (state.pet.hunger + def.hunger_restore).min(100);
    state.pet.mood = (state.pet.mood + def.mood_restore).min(100);
    state.pet.energy = (state.pet.energy + def.energy_restore).min(100);
    state.clamp_pet();
    Ok(format!("{} 吃了{}，精神多了", state.pet.name, def.name))
}

pub fn plant(state: &mut GameState, plot_index: usize, crop_id: &str) -> Result<(), String> {
    refresh(state);
    let plot = state
        .farm
        .plots
        .get_mut(plot_index)
        .ok_or_else(|| "地块不存在".to_string())?;
    if !plot.unlocked {
        return Err("地块未解锁".into());
    }
    if plot.crop.is_some() {
        return Err("地里已有作物".into());
    }
    let _def = crop(crop_id).ok_or_else(|| "未知作物".to_string())?;
    let seed = seed_item_id(crop_id);
    state.take_item(&seed, 1)?;
    let plot = &mut state.farm.plots[plot_index];
    plot.crop = Some(CropInstance {
        crop_id: crop_id.to_string(),
        planted_at: now_secs(),
        watered: false,
        wilted: false,
    });
    Ok(())
}

pub fn water(state: &mut GameState, plot_index: usize) -> Result<(), String> {
    refresh(state);
    let cost = if work_efficiency(state.pet.hunger, state.pet.energy) < 1.0 {
        12
    } else {
        8
    };
    if state.pet.energy < cost {
        return Err("体力不够，先喂点东西或休息一下".into());
    }
    let plot = state
        .farm
        .plots
        .get_mut(plot_index)
        .ok_or_else(|| "地块不存在".to_string())?;
    let crop_inst = plot.crop.as_mut().ok_or_else(|| "空地无需浇水".to_string())?;
    if crop_inst.wilted {
        return Err("已经枯萎了，清空后再种".into());
    }
    if crop_inst.watered {
        return Err("已经浇过水了".into());
    }
    crop_inst.watered = true;
    state.pet.energy -= cost;
    state.clamp_pet();
    Ok(())
}

pub fn clear_wilted(state: &mut GameState, plot_index: usize) -> Result<(), String> {
    refresh(state);
    let plot = state
        .farm
        .plots
        .get_mut(plot_index)
        .ok_or_else(|| "地块不存在".to_string())?;
    match &plot.crop {
        Some(c) if c.wilted => {
            plot.crop = None;
            Ok(())
        }
        _ => Err("没有枯萎作物".into()),
    }
}

pub fn harvest(state: &mut GameState, plot_index: usize) -> Result<String, String> {
    refresh(state);
    let snapshot = state.clone();
    let plot = state
        .farm
        .plots
        .get_mut(plot_index)
        .ok_or_else(|| "地块不存在".to_string())?;
    let crop_inst = plot.crop.as_ref().ok_or_else(|| "空地".to_string())?;
    if crop_inst.wilted {
        return Err("枯萎了，请先清理".into());
    }
    if !is_mature(
        &snapshot,
        crop_inst.planted_at,
        &crop_inst.crop_id,
        crop_inst.watered,
        crop_inst.wilted,
    ) {
        return Err("还没成熟（记得浇水）".into());
    }
    let crop_id = crop_inst.crop_id.clone();
    let def = crop(&crop_id).ok_or_else(|| "未知作物".to_string())?;
    plot.crop = None;
    state.add_item(&harvest_item_id(&crop_id), 1);
    state.total_harvests += 1;
    state.pet.mood = (state.pet.mood + 3).min(100);
    Ok(format!("收获了 {}", def.name))
}

pub fn buy_seed(state: &mut GameState, crop_id: &str, qty: u32) -> Result<(), String> {
    refresh(state);
    if qty == 0 || qty > 99 {
        return Err("数量不合法".into());
    }
    let def = crop(crop_id).ok_or_else(|| "未知作物".to_string())?;
    let cost = def.seed_price.saturating_mul(qty);
    if state.coins < cost {
        return Err("金币不够".into());
    }
    state.coins -= cost;
    state.add_item(&seed_item_id(crop_id), qty);
    Ok(())
}

pub fn sell_item(state: &mut GameState, item_id: &str, qty: u32) -> Result<(), String> {
    refresh(state);
    if qty == 0 {
        return Err("数量不合法".into());
    }
    if !item_id.starts_with("food_") {
        return Err("目前只能卖收获的食物".into());
    }
    let crop_id = item_id.trim_start_matches("food_");
    let def = crop(crop_id).ok_or_else(|| "未知物品".to_string())?;
    state.take_item(item_id, qty)?;
    state.coins += def.sell_price.saturating_mul(qty);
    Ok(())
}

pub fn unlock_plot(state: &mut GameState, plot_index: usize) -> Result<(), String> {
    refresh(state);
    let price = state.unlock_price(plot_index)?;
    if state.coins < price {
        return Err(format!("需要 {price} 金币"));
    }
    state.coins -= price;
    state.farm.plots[plot_index].unlocked = true;
    Ok(())
}
