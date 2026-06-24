#[cfg(target_os = "macos")]
mod macos;

#[tauri::command]
fn trace_status() -> &'static str {
    "Rust backend is connected."
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![trace_status])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            #[cfg(target_os = "macos")]
            {
                macos::apply_traffic_light_inset(app.handle(), "main");
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    for delay_ms in [150, 600] {
                        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                        macos::apply_traffic_light_inset(&handle, "main");
                    }
                });
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                if matches!(
                    event,
                    tauri::WindowEvent::Focused(true) | tauri::WindowEvent::ThemeChanged(_)
                ) {
                    macos::apply_traffic_light_inset(window.app_handle(), window.label());
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
