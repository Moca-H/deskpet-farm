use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! deskpet-farm is alive.")
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_farm_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
