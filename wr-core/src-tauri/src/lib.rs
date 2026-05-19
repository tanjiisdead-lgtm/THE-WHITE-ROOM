use crate::modules::crypto::CryptoManager;
use crate::modules::database::DatabaseManager;
use crate::modules::lockdown::LockdownManager;
use std::sync::Arc;
use tauri::Manager;
use log::{info, error};

pub mod modules;

#[tauri::command]
async fn get_logic_gate(id: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    info!("Fetching curriculum: {}", id);
    state.db.get_curriculum(&id).await.map_err(|e| {
        error!("Failed to fetch curriculum {}: {}", id, e);
        e
    })
}

#[tauri::command]
fn exit_application(app_handle: tauri::AppHandle) {
    info!("Application exit requested.");
    app_handle.exit(0);
}

#[tauri::command]
async fn clear_user_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("Clearing performance logs.");
    state.db.clear_logs().await.map_err(|e| {
        error!("Failed to clear logs: {}", e);
        e
    })
}

pub struct AppState {
    pub crypto: Arc<CryptoManager>,
    pub db: Arc<DatabaseManager>,
    pub lockdown: Arc<LockdownManager>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting W.R. Study Assistant...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_data_path = app.path().app_data_dir().unwrap_or_else(|_| {
                std::env::current_dir().unwrap().join("data")
            });

            tauri::async_runtime::spawn(async move {
                let crypto = match CryptoManager::new(app_data_path.clone()) {
                    Ok(c) => Arc::new(c),
                    Err(e) => {
                        error!("Failed to initialize crypto: {}", e);
                        return;
                    }
                };

                let db = match DatabaseManager::new(crypto.clone(), app_data_path).await {
                    Ok(d) => Arc::new(d),
                    Err(e) => {
                        error!("Failed to initialize database: {}", e);
                        return;
                    }
                };

                let seed_data = include_str!("curriculum_seed.json");
                let v: serde_json::Value = serde_json::from_str(seed_data).unwrap();
                if let Some(problems) = v["curriculum"].as_array() {
                    for p in problems {
                        if let Some(id) = p["id"].as_str() {
                            let content = serde_json::to_vec(p).unwrap();
                            let _ = db.store_curriculum(id, &content).await;
                        }
                    }
                }

                app_handle.manage(AppState {
                    crypto,
                    db,
                    lockdown: Arc::new(LockdownManager::new()),
                });
                info!("Backend services initialized successfully.");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_logic_gate,
            exit_application,
            clear_user_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
