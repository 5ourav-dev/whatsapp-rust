use std::env;
use std::fs;
use std::path::PathBuf;
use tao::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
};
use wry::{WebContext, WebViewBuilder};

const ICON_BYTES: &[u8] = include_bytes!("icon_32.rgba");

struct WindowState {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 1180,
            height: 760,
            x: 100,
            y: 100,
            maximized: false,
        }
    }
}

fn load_window_state(state_file: &PathBuf) -> WindowState {
    if let Ok(content) = fs::read_to_string(state_file) {
        let mut state = WindowState::default();
        for line in content.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let val = parts[1].trim();
                match key {
                    "width" => if let Ok(v) = val.parse() { state.width = v; },
                    "height" => if let Ok(v) = val.parse() { state.height = v; },
                    "x" => if let Ok(v) = val.parse() { state.x = v; },
                    "y" => if let Ok(v) = val.parse() { state.y = v; },
                    "maximized" => state.maximized = val == "true",
                    _ => {}
                }
            }
        }
        return state;
    }
    WindowState::default()
}

fn save_window_state(state_file: &PathBuf, state: &WindowState) {
    let content = format!(
        "width={}\nheight={}\nx={}\ny={}\nmaximized={}\n",
        state.width, state.height, state.x, state.y, state.maximized
    );
    let _ = fs::write(state_file, content);
}

fn main() -> wry::Result<()> {
    let args: Vec<String> = env::args().collect();
    let profile = if args.len() > 1 {
        args[1].clone()
    } else {
        "Personal".to_string()
    };

    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let base_dir = if exe_dir.ends_with("App") || exe_dir.ends_with("app") {
        exe_dir.parent().unwrap_or(&exe_dir).to_path_buf()
    } else {
        exe_dir.clone()
    };

    let data_dir = base_dir.join("Data").join(&profile);
    let _ = fs::create_dir_all(&data_dir);
    let state_file = data_dir.join("window_state.ini");

    let saved_state = load_window_state(&state_file);

    let event_loop = EventLoop::new();
    let title = format!("WhatsApp - {}", profile);

    let window_icon = Icon::from_rgba(ICON_BYTES.to_vec(), 32, 32).ok();

    let mut win_builder = WindowBuilder::new()
        .with_title(&title)
        .with_inner_size(PhysicalSize::new(saved_state.width, saved_state.height))
        .with_maximized(saved_state.maximized);

    // Restore position if not maximized and coordinates are plausible
    if !saved_state.maximized && saved_state.x >= -500 && saved_state.y >= -500 {
        win_builder = win_builder.with_position(PhysicalPosition::new(saved_state.x, saved_state.y));
    }

    if let Some(icon) = window_icon {
        win_builder = win_builder.with_window_icon(Some(icon));
    }

    let window = win_builder.build(&event_loop).expect("Failed to build window");

    let mut web_context = WebContext::new(Some(data_dir.clone()));

    let init_script = r#"
        window.addEventListener('DOMContentLoaded', () => {
            const style = document.createElement('style');
            style.textContent = `body { background: #111b21 !important; }`;
            document.head.appendChild(style);
        });
    "#;

    let _webview = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url("https://web.whatsapp.com")
        .with_initialization_script(init_script)
        .build(&window)
        .expect("Failed to create webview");

    // Keep track of latest un-minimized geometry
    let mut last_size = window.inner_size();
    let mut last_pos = window.outer_position().unwrap_or(PhysicalPosition::new(100, 100));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if !window.is_maximized() && size.width > 200 && size.height > 200 {
                        last_size = size;
                    }
                }
                WindowEvent::Moved(pos) => {
                    if !window.is_maximized() {
                        last_pos = pos;
                    }
                }
                WindowEvent::CloseRequested => {
                    let maximized = window.is_maximized();
                    let state = WindowState {
                        width: last_size.width,
                        height: last_size.height,
                        x: last_pos.x,
                        y: last_pos.y,
                        maximized,
                    };
                    save_window_state(&state_file, &state);
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => (),
        }
    });
}
