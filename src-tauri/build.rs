fn main() {
    // Compile C++ bridge
    let mut build = cc::Build::new();
    
    build
        .cpp(true)
        .file("src/discord_bridge.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-fexceptions");
    
    // Android-specific configuration
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "android" {
        build.flag("-D__ANDROID__");
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=android");
    }
    
    build.compile("discord_bridge");
    
    // Run Tauri build
    tauri_build::build()
}
