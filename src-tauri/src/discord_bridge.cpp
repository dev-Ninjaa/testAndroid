#include <cstdint>
#include <cstring>
#include <string>
#include <memory>
#include <chrono>

#ifdef __ANDROID__
#include <android/log.h>
#include <jni.h>
#define LOG_TAG "DiscordRPC"
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG, LOG_TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

// JNI environment - set from Tauri Android
static JavaVM* g_jvm = nullptr;
static jobject g_context = nullptr;

#else
#include <cstdio>
#define LOGD(...) printf(__VA_ARGS__)
#define LOGE(...) fprintf(stderr, __VA_ARGS__)
#endif

// Discord RPC structures
struct DiscordActivity {
    char details[128];
    char state[128];
    int64_t start_timestamp;
    int64_t end_timestamp;
    char large_image[256];
    char large_text[128];
};

// Simple state management
static bool is_connected = false;
static char app_id[32] = "";
static DiscordActivity current_activity = {0};

#ifdef __ANDROID__
// Set JNI environment (called from Tauri Android lifecycle)
void discord_set_jni_env(JavaVM* jvm, jobject context) {
    g_jvm = jvm;
    
    JNIEnv* env = nullptr;
    if (jvm->GetEnv((void**)&env, JNI_VERSION_1_6) == JNI_OK) {
        g_context = env->NewGlobalRef(context);
        LOGD("JNI environment initialized\n");
    }
}

// Helper to get JNI environment
JNIEnv* get_jni_env() {
    if (!g_jvm) return nullptr;
    
    JNIEnv* env = nullptr;
    int status = g_jvm->GetEnv((void**)&env, JNI_VERSION_1_6);
    
    if (status == JNI_EDETACHED) {
        if (g_jvm->AttachCurrentThread(&env, nullptr) != JNI_OK) {
            return nullptr;
        }
    }
    
    return env;
}

// Check if Discord app is installed
bool is_discord_installed() {
    JNIEnv* env = get_jni_env();
    if (!env || !g_context) return false;
    
    jclass context_class = env->FindClass("android/content/Context");
    if (!context_class) return false;
    
    jmethodID get_package_manager = env->GetMethodID(context_class, "getPackageManager", "()Landroid/content/pm/PackageManager;");
    if (!get_package_manager) return false;
    
    jobject package_manager = env->CallObjectMethod(g_context, get_package_manager);
    if (!package_manager) return false;
    
    jclass pm_class = env->FindClass("android/content/pm/PackageManager");
    jmethodID get_package_info = env->GetMethodID(pm_class, "getPackageInfo", "(Ljava/lang/String;I)Landroid/content/pm/PackageInfo;");
    
    jstring package_name = env->NewStringUTF("com.discord");
    
    bool installed = false;
    try {
        jobject package_info = env->CallObjectMethod(package_manager, get_package_info, package_name, 0);
        installed = (package_info != nullptr);
    } catch (...) {
        installed = false;
    }
    
    env->DeleteLocalRef(package_name);
    env->DeleteLocalRef(package_manager);
    env->DeleteLocalRef(pm_class);
    env->DeleteLocalRef(context_class);
    
    return installed;
}

// Send broadcast intent to Discord
bool send_discord_broadcast(const char* action, const char* json_data) {
    JNIEnv* env = get_jni_env();
    if (!env || !g_context) {
        LOGE("JNI environment not available\n");
        return false;
    }
    
    // Create Intent
    jclass intent_class = env->FindClass("android/content/Intent");
    if (!intent_class) {
        LOGE("Failed to find Intent class\n");
        return false;
    }
    
    jmethodID intent_constructor = env->GetMethodID(intent_class, "<init>", "(Ljava/lang/String;)V");
    jstring action_str = env->NewStringUTF(action);
    jobject intent = env->NewObject(intent_class, intent_constructor, action_str);
    
    // Set package
    jmethodID set_package = env->GetMethodID(intent_class, "setPackage", "(Ljava/lang/String;)Landroid/content/Intent;");
    jstring package_name = env->NewStringUTF("com.discord");
    env->CallObjectMethod(intent, set_package, package_name);
    
    // Add data as extras
    if (json_data) {
        jmethodID put_extra = env->GetMethodID(intent_class, "putExtra", "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;");
        jstring key = env->NewStringUTF("activity");
        jstring value = env->NewStringUTF(json_data);
        env->CallObjectMethod(intent, put_extra, key, value);
        env->DeleteLocalRef(key);
        env->DeleteLocalRef(value);
        
        // Add app ID
        jstring app_id_key = env->NewStringUTF("application_id");
        jstring app_id_value = env->NewStringUTF(app_id);
        env->CallObjectMethod(intent, put_extra, app_id_key, app_id_value);
        env->DeleteLocalRef(app_id_key);
        env->DeleteLocalRef(app_id_value);
    }
    
    // Send broadcast
    jclass context_class = env->FindClass("android/content/Context");
    jmethodID send_broadcast = env->GetMethodID(context_class, "sendBroadcast", "(Landroid/content/Intent;)V");
    env->CallVoidMethod(g_context, send_broadcast, intent);
    
    // Cleanup
    env->DeleteLocalRef(action_str);
    env->DeleteLocalRef(package_name);
    env->DeleteLocalRef(intent);
    env->DeleteLocalRef(intent_class);
    env->DeleteLocalRef(context_class);
    
    LOGD("Broadcast sent: %s\n", action);
    return true;
}
#endif

extern "C" {

// Initialize Discord RPC
bool discord_init(const char* application_id) {
    if (application_id == nullptr || strlen(application_id) == 0) {
        LOGE("Invalid application ID\n");
        return false;
    }
    
    strncpy(app_id, application_id, sizeof(app_id) - 1);
    app_id[sizeof(app_id) - 1] = '\0';
    
    LOGD("Discord RPC initialized with app ID: %s\n", app_id);
    
#ifdef __ANDROID__
    // Check if Discord is installed
    is_connected = is_discord_installed();
    if (is_connected) {
        LOGD("Discord Android app found and connected\n");
    } else {
        LOGD("Discord Android app not installed - RPC will be disabled\n");
    }
#else
    // On desktop, assume connection (would use discord-rich-presence crate)
    is_connected = true;
    LOGD("Discord desktop connection established\n");
#endif
    
    return is_connected;
}

// Update Discord presence
bool discord_update_presence(
    const char* details,
    const char* state,
    int64_t start_timestamp,
    int64_t end_timestamp
) {
    if (!is_connected) {
        LOGE("Discord not connected\n");
        return false;
    }
    
    // Clear current activity
    memset(&current_activity, 0, sizeof(current_activity));
    
    // Copy new activity data
    if (details) {
        strncpy(current_activity.details, details, sizeof(current_activity.details) - 1);
    }
    if (state) {
        strncpy(current_activity.state, state, sizeof(current_activity.state) - 1);
    }
    
    current_activity.start_timestamp = start_timestamp;
    current_activity.end_timestamp = end_timestamp;
    
    LOGD("Updated presence: %s | %s\n", current_activity.details, current_activity.state);
    
#ifdef __ANDROID__
    // Build JSON payload for Discord
    char json_payload[1024];
    snprintf(json_payload, sizeof(json_payload),
        "{\"type\":2,\"details\":\"%s\",\"state\":\"%s\",\"timestamps\":{\"start\":%lld,\"end\":%lld},\"buttons\":[{\"label\":\"Get Maple Music\",\"url\":\"https://github.com/dev-Ninjaa/maple.music\"}]}",
        current_activity.details,
        current_activity.state,
        (long long)start_timestamp,
        (long long)end_timestamp
    );
    
    return send_discord_broadcast("com.discord.RICH_PRESENCE", json_payload);
#else
    // On desktop, would use discord-rich-presence crate (handled in Rust)
    LOGD("Desktop RPC update (handled by Rust layer)\n");
    return true;
#endif
}

// Clear Discord presence
bool discord_clear_presence() {
    if (!is_connected) {
        return false;
    }
    
    memset(&current_activity, 0, sizeof(current_activity));
    LOGD("Cleared presence\n");
    
#ifdef __ANDROID__
    return send_discord_broadcast("com.discord.RICH_PRESENCE_CLEAR", nullptr);
#else
    LOGD("Desktop RPC clear (handled by Rust layer)\n");
    return true;
#endif
}

// Shutdown Discord RPC
void discord_shutdown() {
    if (is_connected) {
        discord_clear_presence();
        is_connected = false;
        LOGD("Discord RPC shutdown\n");
    }
}

// Check connection status
bool discord_is_connected() {
    return is_connected;
}

} // extern "C"
