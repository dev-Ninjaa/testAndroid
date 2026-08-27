#[cfg(mobile)]
mod mobile_impl {
    use super::*;

    #[tauri::mobile_entry_point]
    pub fn mobile_main() {
        crate::run();
    }
}
