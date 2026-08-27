package com.maple.music

import android.os.Bundle
import app.tauri.plugin.PluginManager

class MainActivity : app.tauri.TauriActivity() {
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Initialize native Discord bridge with JNI context
        initializeDiscordBridge()
    }
    
    private external fun initializeDiscordBridge()
    
    companion object {
        init {
            // Load native library
            System.loadLibrary("discord_bridge")
        }
    }
}
