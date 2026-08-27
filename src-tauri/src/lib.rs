use tauri::Manager;
use serde::Deserialize;
use std::sync::Mutex;

mod discord_ffi;

#[cfg(target_os = "android")]
mod jni_bridge;

pub struct AppState {
    discord_handle: Option<discord_ffi::DiscordHandle>,
    current_title: String,
    current_artist: String,
    current_position: f64,
    current_duration: f64,
}

#[derive(Debug, Deserialize)]
struct DiscordPresencePayload {
    title: String,
    artist: String,
    video_id: String,
    duration: f64,
}

#[derive(Debug, Deserialize)]
struct DiscordPositionPayload {
    position: f64,
}

#[derive(Debug, Deserialize)]
struct DiscordPlayingPayload {
    playing: bool,
}

#[tauri::command]
fn update_discord_presence(
    state: tauri::State<Mutex<AppState>>,
    title: String,
    artist: String,
    video_id: String,
    duration: f64,
) -> Result<(), String> {
    let mut app_state = state.lock().map_err(|e| e.to_string())?;
    
    app_state.current_title = title.clone();
    app_state.current_artist = artist.clone();
    app_state.current_duration = duration;
    
    if let Some(handle) = &app_state.discord_handle {
        handle.set_track(&title, &artist, &video_id);
        Ok(())
    } else {
        Ok(()) // Silently succeed if Discord not available
    }
}

#[tauri::command]
fn update_discord_position(
    state: tauri::State<Mutex<AppState>>,
    position: f64,
) -> Result<(), String> {
    let mut app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.current_position = position;
    Ok(())
}

#[tauri::command]
fn discord_set_playing(
    state: tauri::State<Mutex<AppState>>,
    playing: bool,
) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    
    if let Some(handle) = &app_state.discord_handle {
        handle.set_playing(
            playing,
            &app_state.current_title,
            &app_state.current_artist,
            app_state.current_position,
            app_state.current_duration,
        );
        Ok(())
    } else {
        Ok(()) // Silently succeed if Discord not available
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    const APP_ID: &str = "1540597943151763486";
    
    let discord_handle = discord_ffi::DiscordHandle::new(APP_ID);
    
    let app_state = AppState {
        discord_handle: Some(discord_handle),
        current_title: String::new(),
        current_artist: String::new(),
        current_position: 0.0,
        current_duration: 0.0,
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(app_state))
        .invoke_handler(tauri::generate_handler![
            update_discord_presence,
            update_discord_position,
            discord_set_playing
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
