# 🍁 Maple Music

A simple music player with Discord Rich Presence integration built with Tauri for Android.

## Features

- 🎵 Simple and clean music player interface
- 🎶 3 pre-loaded songs for playback
- 🎮 Discord Rich Presence - Shows "Currently Playing Maple Music" on your Discord profile
- 📱 Android APK support via Tauri
- 🌐 Web-based UI using HTML, CSS, and JavaScript

## Discord Rich Presence

When you open the app on your phone and Discord is running on your desktop, your Discord profile will show:
- **Listening to [Song Title]**
- Artist name
- Progress bar with elapsed/remaining time
- "Get Maple Music" button linking to this repository

## Project Structure

```
.
├── Ui/                          # Frontend files
│   ├── index.html              # Main HTML
│   ├── style.css               # Styling
│   └── script.js               # Player logic & Discord integration
├── src-tauri/                  # Tauri Rust backend
│   ├── src/
│   │   ├── main.rs            # Entry point
│   │   ├── lib.rs             # App logic & commands
│   │   └── discord.rs         # Discord Rich Presence implementation
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri configuration
├── discord.rs                  # Original Discord RPC reference
└── .github/workflows/         # CI/CD
    └── android-build.yml      # Android APK build workflow
```

## Building

### Prerequisites

- Rust (stable toolchain)
- Node.js (optional, UI is plain HTML/CSS/JS)
- Java JDK 17+
- Android SDK & NDK
- Tauri CLI v2.0+

### Local Development

```bash
# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0"

# Initialize Tauri for Android
cd src-tauri
cargo tauri android init

# Run on Android (requires connected device/emulator)
cargo tauri android dev

# Build APK
cargo tauri android build --apk
```

### GitHub Actions Build

The project includes a GitHub Actions workflow that automatically builds the Android APK when you push to the repository.

1. Push your code to GitHub
2. Go to Actions tab in your repository
3. The workflow will automatically build the APK
4. Download the APK from the Artifacts section

## Discord Application Setup

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application named "Maple Music"
3. Copy the Application ID
4. Replace the `APP_ID` constant in `src-tauri/src/discord.rs` with your Application ID

**Note:** The current APP_ID in the code is already set. You can use it or replace it with your own.

## Songs

The player comes with 3 demo songs from SoundHelix:
- Chill Vibes
- Summer Breeze  
- Night Dreams

To add your own songs, edit the `songs` array in `Ui/script.js`.

## How Discord Rich Presence Works

The app uses Discord's IPC (Inter-Process Communication) to update your Discord status:

1. When a song plays, the frontend calls the Tauri backend
2. The Rust backend communicates with Discord via the `discord-rich-presence` crate
3. Your Discord profile shows the currently playing song with:
   - Song title and artist
   - Playback progress
   - Elapsed and total duration
   - Clickable buttons

**Note:** Discord Rich Presence works when Discord desktop is running on the same machine. On Android, the app will attempt to connect but may not show presence if Discord is only running on mobile.

## Technologies

- **Frontend:** HTML5, CSS3, Vanilla JavaScript
- **Backend:** Rust, Tauri 2.0
- **Discord Integration:** discord-rich-presence crate
- **Build System:** GitHub Actions
- **Target Platform:** Android (APK)

## License

Open source - feel free to use and modify!

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
