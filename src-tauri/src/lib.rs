mod game;

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder,
};

use game::achievements::{list_views as achievement_views, AchievementView};
use game::catalog::CROPS;
use game::rules::{
    buy_seed, clear_wilted, feed, harvest, pat, plant, refresh, rest, sell_item, set_pet_name,
    unlock_plot, water,
};
use game::save::{export_to, import_from, load_or_default, save_path, save_state};
use game::state::GameState;
use game::tick::{plot_views, PlotView};

struct AppState {
    game: Mutex<GameState>,
    tray: Mutex<Option<TrayIcon>>,
    known_ready: Mutex<HashSet<usize>>,
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
        let _ = farm.set_size(tauri::LogicalSize::new(1024.0, 780.0));
        farm.unminimize().map_err(|e| e.to_string())?;
        farm.show().map_err(|e| e.to_string())?;
        farm.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    WebviewWindowBuilder::new(&app, "farm", WebviewUrl::App("/farm".into()))
        .title("deskpet farm")
        .inner_size(1024.0, 780.0)
        .center()
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn cmd_rest(app: AppHandle, state: State<AppState>) -> Result<(GameState, String), String> {
    with_game(&app, &state, |g| {
        let msg = rest(g)?;
        Ok((g.clone(), msg))
    })
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FarmSync {
    state: GameState,
    plots: Vec<PlotView>,
    achievements: Vec<AchievementView>,
}

#[tauri::command]
fn farm_sync(app: AppHandle, state: State<AppState>) -> Result<FarmSync, String> {
    with_game(&app, &state, |g| {
        refresh(g);
        Ok(FarmSync {
            state: g.clone(),
            plots: plot_views(g),
            achievements: achievement_views(g),
        })
    })
}

#[tauri::command]
fn list_achievements(app: AppHandle, state: State<AppState>) -> Result<Vec<AchievementView>, String> {
    with_game(&app, &state, |g| {
        refresh(g);
        Ok(achievement_views(g))
    })
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct CropReadyEvent {
    message: String,
    count: usize,
}

fn check_crop_alerts(app: &AppHandle, notify: bool) {
    let Some(state) = app.try_state::<AppState>() else {
        return;
    };
    let mut g = match state.game.lock() {
        Ok(g) => g,
        Err(_) => return,
    };
    let before_tick = g.last_tick_at;
    refresh(&mut g);
    let tick_moved = g.last_tick_at != before_tick;
    let views = plot_views(&g);
    let ready: Vec<&PlotView> = views.iter().filter(|p| p.phase == "ready").collect();
    let ready_idx: HashSet<usize> = ready.iter().map(|p| p.index).collect();
    let mut known = match state.known_ready.lock() {
        Ok(k) => k,
        Err(_) => return,
    };
    let mut did_notify = false;
    if notify {
        let newly: Vec<&PlotView> = ready
            .iter()
            .copied()
            .filter(|p| !known.contains(&p.index))
            .collect();
        if !newly.is_empty() {
            did_notify = true;
            let names: Vec<&str> = newly
                .iter()
                .filter_map(|p| p.crop_name.as_deref())
                .collect();
            let message = if names.is_empty() {
                format!("有 {} 块地可以收获了", newly.len())
            } else {
                format!("{} 成熟了，可以收获", names.join("、"))
            };
            let _ = app.emit(
                "crop-ready",
                CropReadyEvent {
                    message: message.clone(),
                    count: newly.len(),
                },
            );
            if let Ok(tray_lock) = state.tray.lock() {
                if let Some(tray) = tray_lock.as_ref() {
                    let _ = tray.set_tooltip(Some(&format!("deskpet-farm · {message}")));
                }
            }
        }
    }
    if ready.is_empty() {
        if let Ok(tray_lock) = state.tray.lock() {
            if let Some(tray) = tray_lock.as_ref() {
                let _ = tray.set_tooltip(Some("deskpet-farm"));
            }
        }
    }
    *known = ready_idx;
    if tick_moved || did_notify {
        let _ = save_state(app, &g);
    }
}

#[tauri::command]
fn export_save(app: AppHandle, state: State<AppState>, path: String) -> Result<(), String> {
    with_game(&app, &state, |g| {
        refresh(g);
        Ok(())
    })?;
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

fn setup_tray(app: &AppHandle) -> tauri::Result<TrayIcon> {
    let show_pet = MenuItem::with_id(app, "show_pet", "显示桌宠", true, None::<&str>)?;
    let show_farm = MenuItem::with_id(app, "show_farm", "打开农场", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_pet, &show_farm, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("deskpet-farm")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show_pet" => {
                if let Some(w) = app.get_webview_window("pet") {
                    let _ = w.show();
                }
            }
            "show_farm" => {
                let _ = open_farm_window(app.clone());
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
                }
            }
        })
        .build(app)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let state = load_or_default(app.handle()).map_err(|e| {
                Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error>
            })?;
            let tray = setup_tray(app.handle())?;
            app.manage(AppState {
                game: Mutex::new(state),
                tray: Mutex::new(Some(tray)),
                known_ready: Mutex::new(HashSet::new()),
            });
            if let Some(pet) = app.get_webview_window("pet") {
                let _ = pet.set_ignore_cursor_events(false);
                let _ = pet.set_background_color(Some(tauri::window::Color(0, 0, 0, 0)));
                let _ = pet.show();
                let _ = pet.set_focus();
            }
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                // Seed known-ready without notifying on first pass.
                std::thread::sleep(Duration::from_secs(2));
                check_crop_alerts(&handle, false);
                loop {
                    std::thread::sleep(Duration::from_secs(8));
                    check_crop_alerts(&handle, true);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            list_crops,
            list_achievements,
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
            cmd_rest,
            farm_sync,
            open_farm_window,
            export_save,
            import_save,
            save_file_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
