mod game;

use std::path::PathBuf;
use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder,
};

use game::catalog::CROPS;
use game::rules::{
    buy_seed, clear_wilted, feed, harvest, pat, plant, refresh, sell_item, set_pet_name,
    unlock_plot, water,
};
use game::save::{export_to, import_from, load_or_default, save_path, save_state};
use game::state::GameState;

struct AppState {
    game: Mutex<GameState>,
}

fn with_game<T>(
    app: &AppHandle,
    state: &State<'_, AppState>,
    f: impl FnOnce(&mut GameState) -> Result<T, String>,
) -> Result<T, String> {
    let mut g = state.game.lock().map_err(|_| "状态锁失败".to_string())?;
    let out = f(&mut g)?;
    save_state(app, &g)?;
    Ok(out)
}

#[tauri::command]
fn get_state(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        refresh(g);
        Ok(g.clone())
    })
}

#[tauri::command]
fn list_crops() -> Vec<CropDefJson> {
    CROPS
        .iter()
        .map(|c| CropDefJson {
            id: c.id.to_string(),
            name: c.name.to_string(),
            kind: c.kind.to_string(),
            seed_price: c.seed_price,
            sell_price: c.sell_price,
            grow_secs: c.grow_secs,
            wilt_secs: c.wilt_secs,
            hunger_restore: c.hunger_restore,
            mood_restore: c.mood_restore,
            energy_restore: c.energy_restore,
        })
        .collect()
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct CropDefJson {
    id: String,
    name: String,
    kind: String,
    seed_price: u32,
    sell_price: u32,
    grow_secs: u64,
    wilt_secs: u64,
    hunger_restore: i32,
    mood_restore: i32,
    energy_restore: i32,
}

#[tauri::command]
fn cmd_set_pet_name(app: AppHandle, state: State<AppState>, name: String) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        set_pet_name(g, name)?;
        Ok(g.clone())
    })
}

#[tauri::command]
fn cmd_pat(app: AppHandle, state: State<AppState>) -> Result<(GameState, String), String> {
    with_game(&app, &state, |g| {
        let msg = pat(g)?;
        Ok((g.clone(), msg))
    })
}

#[tauri::command]
fn cmd_feed(
    app: AppHandle,
    state: State<AppState>,
    item_id: String,
) -> Result<(GameState, String), String> {
    with_game(&app, &state, |g| {
        let msg = feed(g, &item_id)?;
        Ok((g.clone(), msg))
    })
}

#[tauri::command]
fn cmd_plant(
    app: AppHandle,
    state: State<AppState>,
    plot_index: usize,
    crop_id: String,
) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        plant(g, plot_index, &crop_id)?;
        Ok(g.clone())
    })
}

#[tauri::command]
fn cmd_water(app: AppHandle, state: State<AppState>, plot_index: usize) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        water(g, plot_index)?;
        Ok(g.clone())
    })
}

#[tauri::command]
fn cmd_harvest(
    app: AppHandle,
    state: State<AppState>,
    plot_index: usize,
) -> Result<(GameState, String), String> {
    with_game(&app, &state, |g| {
        let msg = harvest(g, plot_index)?;
        Ok((g.clone(), msg))
    })
}

#[tauri::command]
fn cmd_clear_wilted(
    app: AppHandle,
    state: State<AppState>,
    plot_index: usize,
) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        clear_wilted(g, plot_index)?;
        Ok(g.clone())
    })
}

#[tauri::command]
fn cmd_buy_seed(
    app: AppHandle,
    state: State<AppState>,
    crop_id: String,
    qty: u32,
) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        buy_seed(g, &crop_id, qty)?;
        Ok(g.clone())
    })
}

#[tauri::command]
fn cmd_sell_item(
    app: AppHandle,
    state: State<AppState>,
    item_id: String,
    qty: u32,
) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        sell_item(g, &item_id, qty)?;
        Ok(g.clone())
    })
}

#[tauri::command]
fn cmd_unlock_plot(
    app: AppHandle,
    state: State<AppState>,
    plot_index: usize,
) -> Result<GameState, String> {
    with_game(&app, &state, |g| {
        unlock_plot(g, plot_index)?;
        Ok(g.clone())
    })
}

#[tauri::command]
fn open_farm_window(app: AppHandle) -> Result<(), String> {
    if let Some(farm) = app.get_webview_window("farm") {
        farm.show().map_err(|e| e.to_string())?;
        farm.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    WebviewWindowBuilder::new(&app, "farm", WebviewUrl::App("/farm".into()))
        .title("deskpet farm")
        .inner_size(960.0, 640.0)
        .center()
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn set_click_through(app: AppHandle, enabled: bool) -> Result<(), String> {
    if let Some(pet) = app.get_webview_window("pet") {
        pet.set_ignore_cursor_events(enabled)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn export_save(app: AppHandle, path: String) -> Result<(), String> {
    export_to(&app, &PathBuf::from(path))
}

#[tauri::command]
fn import_save(app: AppHandle, state: State<AppState>, path: String) -> Result<GameState, String> {
    let loaded = import_from(&app, &PathBuf::from(path))?;
    let mut g = state.game.lock().map_err(|_| "状态锁失败".to_string())?;
    *g = loaded.clone();
    Ok(loaded)
}

#[tauri::command]
fn save_file_path(app: AppHandle) -> Result<String, String> {
    Ok(save_path(&app)?.to_string_lossy().to_string())
}

fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let show_pet = MenuItem::with_id(app, "show_pet", "显示桌宠", true, None::<&str>)?;
    let show_farm = MenuItem::with_id(app, "show_farm", "打开农场", true, None::<&str>)?;
    let interact = MenuItem::with_id(app, "interact", "允许点击桌宠", true, None::<&str>)?;
    let pass = MenuItem::with_id(app, "passthrough", "点击穿透", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_pet, &show_farm, &interact, &pass, &quit])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("deskpet-farm")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show_pet" => {
                if let Some(w) = app.get_webview_window("pet") {
                    let _ = w.show();
                    let _ = w.set_ignore_cursor_events(false);
                }
            }
            "show_farm" => {
                let _ = open_farm_window(app.clone());
            }
            "interact" => {
                if let Some(w) = app.get_webview_window("pet") {
                    let _ = w.set_ignore_cursor_events(false);
                    let _ = w.show();
                }
            }
            "passthrough" => {
                if let Some(w) = app.get_webview_window("pet") {
                    let _ = w.set_ignore_cursor_events(true);
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(w) = app.get_webview_window("pet") {
                    let _ = w.show();
                    let _ = w.set_ignore_cursor_events(false);
                }
            }
        })
        .build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = load_or_default(app.handle()).map_err(|e| {
                Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error>
            })?;
            app.manage(AppState {
                game: Mutex::new(state),
            });
            setup_tray(app.handle())?;
            // Default: clickable. Use tray "点击穿透" when you need the pet to not block the mouse.
            if let Some(pet) = app.get_webview_window("pet") {
                let _ = pet.set_ignore_cursor_events(false);
                let _ = pet.show();
                let _ = pet.set_focus();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            list_crops,
            cmd_set_pet_name,
            cmd_pat,
            cmd_feed,
            cmd_plant,
            cmd_water,
            cmd_harvest,
            cmd_clear_wilted,
            cmd_buy_seed,
            cmd_sell_item,
            cmd_unlock_plot,
            open_farm_window,
            set_click_through,
            export_save,
            import_save,
            save_file_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
