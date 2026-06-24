use objc2_app_kit::{NSWindow, NSWindowButton};
use tauri::Manager;

pub fn apply_traffic_light_inset(app: &tauri::AppHandle, label: &str) {
    let config = app.config();
    let Some(window_config) = config
        .app
        .windows
        .iter()
        .find(|window| window.label == label)
    else {
        return;
    };
    let Some(position) = &window_config.traffic_light_position else {
        return;
    };
    let Some(window) = app.get_webview_window(label) else {
        return;
    };
    let Ok(ns_window_ptr) = window.ns_window() else {
        return;
    };

    let addr = ns_window_ptr as usize;
    let (x, y) = (position.x, position.y);
    let _ = window.run_on_main_thread(move || unsafe {
        inset_traffic_lights(&*(addr as *const NSWindow), x, y);
    });
}

unsafe fn inset_traffic_lights(window: &NSWindow, x: f64, y: f64) {
    unsafe {
        let Some(close) = window.standardWindowButton(NSWindowButton::CloseButton) else {
            return;
        };
        let Some(minimize) = window.standardWindowButton(NSWindowButton::MiniaturizeButton) else {
            return;
        };
        let Some(zoom) = window.standardWindowButton(NSWindowButton::ZoomButton) else {
            return;
        };
        let Some(container) = close.superview().and_then(|view| view.superview()) else {
            return;
        };

        let close_frame = close.frame();
        let bar_height = close_frame.size.height + y;
        let mut bar_frame = container.frame();
        bar_frame.size.height = bar_height;
        bar_frame.origin.y = window.frame().size.height - bar_height;
        container.setFrame(bar_frame);

        let spacing = minimize.frame().origin.x - close_frame.origin.x;
        for (index, button) in [&*close, &*minimize, &*zoom].into_iter().enumerate() {
            let mut frame = button.frame();
            frame.origin.x = x + index as f64 * spacing;
            button.setFrameOrigin(frame.origin);
        }
    }
}
