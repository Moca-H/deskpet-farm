use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use super::state::{GameState, SAVE_VERSION};
use super::tick::catch_up;

pub fn save_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("app data dir: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("save.json"))
}

pub fn load_or_default(app: &AppHandle) -> Result<GameState, String> {
    let path = save_path(app)?;
    let mut state = if path.exists() {
        let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut s: GameState = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        if s.version > SAVE_VERSION {
            return Err("存档版本过新，请更新应用".into());
        }
        s.version = SAVE_VERSION;
        if s.farm.plots.len() != 9 {
            s.farm.plots.resize(9, super::state::Plot {
                unlocked: false,
                crop: None,
            });
        }
        s
    } else {
        GameState::default()
    };
    catch_up(&mut state);
    save_state(app, &state)?;
    Ok(state)
}

pub fn save_state(app: &AppHandle, state: &GameState) -> Result<(), String> {
    let path = save_path(app)?;
    let raw = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    fs::write(path, raw).map_err(|e| e.to_string())
}

pub fn export_to(app: &AppHandle, dest: &PathBuf) -> Result<(), String> {
    let src = save_path(app)?;
    if !src.exists() {
        return Err("还没有存档".into());
    }
    fs::copy(src, dest).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn import_from(app: &AppHandle, src: &PathBuf) -> Result<GameState, String> {
    let raw = fs::read_to_string(src).map_err(|e| e.to_string())?;
    let mut state: GameState = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    if state.version > SAVE_VERSION {
        return Err("存档版本过新".into());
    }
    state.version = SAVE_VERSION;
    catch_up(&mut state);
    save_state(app, &state)?;
    Ok(state)
}
