use crate::store::initialize_settings;
use tauri::Manager;
pub mod ai_translator;
pub mod shell_helper;
pub mod shortcut;
pub mod store;
pub mod tray;

#[tauri::command]
fn log_to_backend(message: String) {
    println!("Frontend Log: {}", message);
}

#[tauri::command]
fn get_version(app_handle: tauri::AppHandle) -> String {
    app_handle.package_info().version.to_string()
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn update_translator_shortcut(
    app_handle: tauri::AppHandle,
    keys: Vec<String>,
) -> Result<(), String> {
    shortcut::update_translator_shortcut(&app_handle, keys)
}

#[tauri::command]
async fn translate_text(app_handle: tauri::AppHandle, text: String) -> Result<String, String> {
    ai_translator::translate_with_gpt(&app_handle, &text)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_settings(app_handle: tauri::AppHandle) -> Result<store::AppSettings, String> {
    store::get_settings(&app_handle).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_system_model_configs() -> std::collections::BTreeMap<String, store::ModelConfig> {
    ai_translator::get_system_model_configs()
}

#[tauri::command]
fn get_config_dir_hint() -> &'static str {
    ai_translator::get_config_dir_hint()
}

pub fn run() {
    println!("Starting application...");
    
    // 加载顺序：exe同级目录 > 项目根目录（开发时）
    let mut env_loaded = false;
    
    // 1. 尝试从exe同级目录加载 .env（打包发布后）
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let env_file = exe_dir.join(".env");
            if env_file.exists() {
                match dotenvy::from_filename(&env_file) {
                    Ok(_) => {
                        println!("✓ 从exe同级目录加载 .env 成功: {:?}", env_file);
                        env_loaded = true;
                    }
                    Err(e) => println!("⚠ 加载 .env 失败: {}", e),
                }
            }
        }
    }
    
    // 2. 如果还未加载，尝试从项目根目录加载（开发时）
    if !env_loaded && std::path::Path::new(".env").exists() {
        println!("✓ 从项目根目录加载 .env");
        let _ = dotenvy::dotenv();
        env_loaded = true;
    }
    
    if !env_loaded {
        println!("⚠ 未找到 .env 文件。请将 .env 文件放在应用目录同级位置");
    }

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        // 剪贴板插件
        .plugin(tauri_plugin_clipboard_manager::init())
        // opener插件
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 初始化存储
            println!("Initializing...");
            match initialize_settings(&app.app_handle()) {
                Ok(_) => println!("应用设置初始化完成"),
                Err(e) => eprintln!("初始化设置失败: {}", e),
            }

            // 初始化所有快捷键
            println!("正在注册全局快捷键...");
            match shortcut::init_shortcuts(&app.app_handle()) {
                Ok(_) => println!("快捷键设置成功"),
                Err(e) => eprintln!("注册全局快捷键失败: {}", e),
            }

            // 创建AI模型托盘
            match tray::create_tray(&app.app_handle()) {
                Ok(_) => println!("托盘创建成功"),
                Err(e) => eprintln!("创建托盘失败: {}", e),
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            update_translator_shortcut,
            log_to_backend,
            get_settings,
            get_system_model_configs,
            get_config_dir_hint,
            get_version,
            translate_text
        ]);

    // 只在非Windows系统上添加窗口事件监听
    #[cfg(not(target_os = "windows"))]
    {
        builder = builder.on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                #[cfg(target_os = "macos")]
                let _ = window
                    .app_handle()
                    .set_activation_policy(tauri::ActivationPolicy::Accessory);
                api.prevent_close();
            }
            _ => {}
        });
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
