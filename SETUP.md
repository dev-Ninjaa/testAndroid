# Maple Music - Complete Setup Guide

## 📱 Building Android APK via GitHub Actions (Recommended)

This is the easiest way to build your APK without setting up the development environment locally.

### Steps:

1. **Push to GitHub:**
   ```bash
   git init
   git add .
   git commit -m "Initial commit - Maple Music Player"
   git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO.git
   git push -u origin main
   ```

2. **Enable GitHub Actions:**
   - Go to your repository on GitHub
   - Click on the "Actions" tab
   - If Actions are disabled, enable them

3. **Workflow will automatically run:**
   - Every push to `main` or `master` branch will trigger the build
   - You can also manually trigger it from the Actions tab (click "Run workflow")

4. **Download the APK:**
   - Go to the Actions tab
   - Click on the latest workflow run
   - Scroll down to "Artifacts"
   - Download `maple-music-android-apk` or `maple-music-android-apk-debug`
   - Extract the ZIP file to get your APK

5. **Install on Android:**
   - Transfer the APK to your Android device
   - Enable "Install from Unknown Sources" in your device settings
   - Open the APK file to install

## 🔧 Local Development Setup (Optional)

If you want to build locally or develop further:

### Prerequisites:

1. **Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Android Studio & SDK:**
   - Download from: https://developer.android.com/studio
   - Install Android SDK (API 33+)
   - Install Android NDK (version 26.1.10909125)

3. **Java JDK 17:**
   - Download from: https://adoptium.net/
   - Or use: `sudo apt install openjdk-17-jdk` (Linux)

4. **Tauri CLI:**
   ```bash
   cargo install tauri-cli --version "^2.0.0"
   ```

### Environment Variables:

```bash
# Add to ~/.bashrc or ~/.zshrc
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/26.1.10909125
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64  # Adjust path
```

### Build Commands:

```bash
# Initialize Android project (first time only)
cd src-tauri
cargo tauri android init

# Build APK
cargo tauri android build --apk

# Build and run on connected device/emulator
cargo tauri android dev
```

### Output Location:
```
src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk
```

## 🎮 Discord Rich Presence Setup

### Getting Your Discord Application ID:

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application"
3. Name it "Maple Music" (this name appears in the Discord status)
4. Click on your application
5. Copy the "Application ID"

### Update the Code:

Open `src-tauri/src/discord.rs` and replace:
```rust
const APP_ID: &str = "YOUR_APPLICATION_ID_HERE";
```

**Note:** The current APP_ID (`1540597943151763486`) is already configured and should work. You only need to change it if you want your own custom Discord application.

### Optional - Custom Icon:

1. In Discord Developer Portal, go to your application
2. Click "Rich Presence" → "Art Assets"
3. Upload an image named "large_image"
4. This will show as the thumbnail in Discord presence

## 📁 Project File Structure

```
maple.music/
├── .github/
│   └── workflows/
│       └── android-build.yml      # GitHub Actions workflow
├── Ui/                            # Frontend (HTML/CSS/JS)
│   ├── index.html                 # Main UI
│   ├── script.js                  # Player logic
│   └── style.css                  # Styles
├── src-tauri/                     # Rust backend
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # Tauri commands
│   │   ├── discord.rs            # Discord RPC
│   │   └── mobile.rs             # Mobile specific code
│   ├── icons/                     # App icons
│   ├── gen/                       # Generated Android project
│   ├── Cargo.toml                 # Rust dependencies
│   ├── tauri.conf.json           # Tauri config
│   └── build.rs                   # Build script
├── discord.rs                     # Original Discord reference
├── Cargo.toml                     # Workspace config
├── README.md                      # Documentation
├── SETUP.md                       # This file
└── .gitignore                     # Git ignore rules
```

## 🎵 Customizing Songs

Edit `Ui/script.js` and modify the `songs` array:

```javascript
const songs = [
    {
        title: "Your Song Title",
        artist: "Artist Name",
        url: "https://example.com/song.mp3",
        videoId: "unique_id_1"
    },
    // Add more songs...
];
```

## 🐛 Troubleshooting

### Build Fails on GitHub Actions:

- **Check Actions tab** for detailed error logs
- **NDK version mismatch:** The workflow uses NDK 26.1.10909125
- **Rust target missing:** Workflow auto-installs Android targets

### Discord RPC Not Working:

- **Discord must be running** on desktop (Rich Presence works via IPC)
- **Application ID must be valid** (digits only, from Discord Developer Portal)
- **Android limitation:** Discord RPC typically works when Discord desktop is on the same network or via desktop app, not Android-to-Android

### APK Installation Fails:

- **Enable Unknown Sources** in Android settings
- **Architecture mismatch:** Use universal APK (works on all architectures)
- **Minimum SDK:** Requires Android 7.0 (API 24) or higher

### Local Build Issues:

```bash
# Clear cache and rebuild
cd src-tauri
cargo clean
cargo tauri android build --apk

# If Android init fails
rm -rf gen/android
cargo tauri android init
```

## 📊 GitHub Actions Workflow Details

The workflow (`android-build.yml`):
- Triggers on: Push to main/master, PRs, manual dispatch
- Sets up: Java 17, Rust, Android SDK/NDK, Tauri CLI
- Builds: Universal APK (works on all Android architectures)
- Uploads: APK as downloadable artifact
- Duration: ~15-30 minutes depending on GitHub runners

### Manual Trigger:

1. Go to Actions tab
2. Select "Build Android APK" workflow
3. Click "Run workflow" button
4. Select branch and run

## 🚀 Quick Start Summary

**Easiest Method (No local setup):**
1. Push code to GitHub
2. Wait for Actions to complete (~20 min)
3. Download APK from Artifacts
4. Install on Android device
5. Open app, Discord will show your status!

**For Development:**
1. Setup Rust + Android SDK + NDK
2. `cargo tauri android init`
3. `cargo tauri android dev`
4. Test on device/emulator

## 📝 Notes

- **Desktop vs Mobile:** Discord Rich Presence primarily works with Discord desktop client
- **Battery Usage:** RPC connection attempts may use battery; the app implements smart backoff
- **No Servers:** All music playback is direct from URLs, no backend server needed
- **Open Source:** Feel free to modify and extend!

## 🤝 Contributing

Contributions welcome! Feel free to:
- Add more features
- Improve UI/UX
- Add more music sources
- Enhance Discord integration
- Report bugs

## 📄 License

Open source - use freely!

---

**Happy Listening! 🎵🍁**
