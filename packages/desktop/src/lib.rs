use tauri::App;
use tokio::sync::Mutex;

pub struct TauriState;

pub fn setup_tauri(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> anyhow::Result<()> {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .manage(Mutex::new(TauriState {}))
    .invoke_handler(tauri::generate_handler![])
    .setup(setup_tauri)
    .run(tauri::generate_context!())
    .map_err(|e| e.into())
}
