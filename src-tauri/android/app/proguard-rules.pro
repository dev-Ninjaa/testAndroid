# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep MainActivity
-keep class com.maple.music.MainActivity { *; }

# Keep Tauri classes
-keep class app.tauri.** { *; }
