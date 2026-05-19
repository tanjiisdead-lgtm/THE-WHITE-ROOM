use crate::modules::crypto::CryptoManager;
use crate::modules::database::DatabaseManager;
use crate::modules::lockdown::LockdownManager;
use std::sync::Arc;
use tauri::Manager;

pub mod modules;

#[tauri::command]
async fn get_logic_gate(id: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    state.db.get_curriculum(&id).await
}

#[tauri::command]
fn trigger_protocol_zero() {
    // Immediate system-level response to failure
    std::process::exit(1);
}

pub struct AppState {
    pub crypto: Arc<CryptoManager>,
    pub db: Arc<DatabaseManager>,
    pub lockdown: Arc<LockdownManager>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let crypto = Arc::new(CryptoManager::new());
    let lockdown = Arc::new(LockdownManager::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let crypto_clone = crypto.clone();

            tauri::async_runtime::block_on(async move {
                let db = Arc::new(DatabaseManager::new(crypto_clone.clone()).await);

                // Seed database with curriculum if empty
                let seed_data = include_str!("curriculum_seed.json");
                let v: serde_json::Value = serde_json::from_str(seed_data).unwrap();
                if let Some(problems) = v["curriculum"].as_array() {
                    for p in problems {
                        let id = p["id"].as_str().unwrap();
                        let content = serde_json::to_vec(p).unwrap();
                        db.store_curriculum(id, &content).await;
                    }
                }

                app_handle.manage(AppState {
                    crypto: crypto_clone,
                    db,
                    lockdown: lockdown.clone(),
                });
            });

            lockdown.enforce_lockdown();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_logic_gate,
            trigger_protocol_zero
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
