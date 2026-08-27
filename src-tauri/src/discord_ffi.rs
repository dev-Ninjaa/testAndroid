use std::ffi::CString;
use std::os::raw::c_char;

#[cfg(target_os = "android")]
use std::os::raw::c_void;

// FFI bindings to C++ Discord bridge
extern "C" {
    fn discord_init(application_id: *const c_char) -> bool;
    fn discord_update_presence(
        details: *const c_char,
        state: *const c_char,
        start_timestamp: i64,
        end_timestamp: i64,
    ) -> bool;
    fn discord_clear_presence() -> bool;
    fn discord_shutdown();
    fn discord_is_connected() -> bool;
    
    #[cfg(target_os = "android")]
    fn discord_set_jni_env(jvm: *mut c_void, context: *mut c_void);
}

#[cfg(target_os = "android")]
pub fn init_jni_env(jvm: *mut c_void, context: *mut c_void) {
    unsafe {
        discord_set_jni_env(jvm, context);
    }
}

pub struct DiscordBridge {
    _initialized: bool,
}

impl DiscordBridge {
    pub fn new(app_id: &str) -> Option<Self> {
        let c_app_id = CString::new(app_id).ok()?;
        
        unsafe {
            if discord_init(c_app_id.as_ptr()) {
                Some(DiscordBridge { _initialized: true })
            } else {
                None
            }
        }
    }
    
    pub fn update_presence(
        &self,
        details: &str,
        state: &str,
        start_timestamp: i64,
        end_timestamp: i64,
    ) -> bool {
        let c_details = CString::new(details).unwrap_or_default();
        let c_state = CString::new(state).unwrap_or_default();
        
        unsafe {
            discord_update_presence(
                c_details.as_ptr(),
                c_state.as_ptr(),
                start_timestamp,
                end_timestamp,
            )
        }
    }
    
    pub fn clear_presence(&self) -> bool {
        unsafe { discord_clear_presence() }
    }
    
    pub fn is_connected(&self) -> bool {
        unsafe { discord_is_connected() }
    }
}

impl Drop for DiscordBridge {
    fn drop(&mut self) {
        unsafe {
            discord_shutdown();
        }
    }
}

// Thread-safe handle for use across the app
use std::sync::{Arc, Mutex};

pub struct DiscordHandle {
    bridge: Arc<Mutex<Option<DiscordBridge>>>,
}

impl DiscordHandle {
    pub fn new(app_id: &str) -> Self {
        let bridge = DiscordBridge::new(app_id);
        DiscordHandle {
            bridge: Arc::new(Mutex::new(bridge)),
        }
    }
    
    pub fn set_track(&self, title: &str, artist: &str, _video_id: &str) {
        // Store for later use with timestamps
        if let Ok(bridge) = self.bridge.lock() {
            if let Some(b) = bridge.as_ref() {
                let _ = b.update_presence(title, artist, 0, 0);
            }
        }
    }
    
    pub fn set_playing(&self, playing: bool, title: &str, artist: &str, position: f64, duration: f64) {
        if let Ok(bridge) = self.bridge.lock() {
            if let Some(b) = bridge.as_ref() {
                if playing {
                    let now_ms = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as i64;
                    
                    let start_ms = now_ms - (position * 1000.0) as i64;
                    let end_ms = if duration > 0.0 {
                        start_ms + (duration * 1000.0) as i64
                    } else {
                        0
                    };
                    
                    let _ = b.update_presence(title, artist, start_ms, end_ms);
                } else {
                    let _ = b.clear_presence();
                }
            }
        }
    }
}

impl Clone for DiscordHandle {
    fn clone(&self) -> Self {
        DiscordHandle {
            bridge: Arc::clone(&self.bridge),
        }
    }
}
