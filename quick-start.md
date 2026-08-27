# 🚀 Quick Start - Maple Music

## Fastest Way to Get Your APK

### Option 1: GitHub Actions (Recommended - No Setup Needed!)

```bash
# 1. Push to GitHub
git init
git add .
git commit -m "🎵 Maple Music - Initial commit"
git branch -M main
git remote add origin https://github.com/YOUR_USERNAME/maple-music.git
git push -u origin main

# 2. Go to GitHub repository → Actions tab
# 3. Wait for build to complete (~20 minutes)
# 4. Download APK from Artifacts
# 5. Install on your Android phone!
```

### Option 2: Local Build (Requires Setup)

```bash
# Prerequisites: Rust, Android SDK, NDK, Java 17
cargo install tauri-cli --version "^2.0.0"

# Build
cd src-tauri
cargo tauri android init
cargo tauri android build --apk

# APK Location:
# gen/android/app/build/outputs/apk/universal/release/*.apk
```

## What You Get

✅ **Simple Music Player** with 3 demo songs  
✅ **Discord Rich Presence** - Shows "Currently Playing" on your Discord  
✅ **Android APK** - Ready to install  
✅ **Fully Customizable** - HTML/CSS/JS frontend  

## File Structure Overview

```
📁 Your Project
├── 📁 Ui/                    ← Your music player (HTML/CSS/JS)
├── 📁 src-tauri/             ← Rust backend + Discord integration
├── 📁 .github/workflows/     ← Auto-build APK on push
└── 📄 README.md              ← Full documentation
```

## Customize Your Player

**Add Your Own Songs** (Edit `Ui/script.js`):
```javascript
const songs = [
    {
        title: "My Awesome Song",
        artist: "Cool Artist",
        url: "https://example.com/song.mp3",
        videoId: "song_1"
    }
];
```

**Change Discord App Name:**
1. Create app at: https://discord.com/developers/applications
2. Copy Application ID
3. Edit `src-tauri/src/discord.rs`:
   ```rust
   const APP_ID: &str = "YOUR_APP_ID";
   ```

## Testing Locally (Without Building APK)

Open `Ui/index.html` directly in your browser to test the player UI.  
(Discord features require the Tauri app to be running)

## Need Help?

- **Full Setup Guide:** See `SETUP.md`
- **Detailed README:** See `README.md`
- **Build Issues:** Check GitHub Actions logs
- **Discord Issues:** Ensure Discord desktop is running

## Important Notes

🔥 **Discord RPC works best with Discord desktop app running**  
📱 **Minimum Android:** Version 7.0 (API 24)  
⚡ **Build Time:** 15-30 minutes on GitHub Actions  
🎵 **Demo Songs:** Using SoundHelix (royalty-free)  

---

**Ready to rock! 🎸 Push to GitHub and get your APK! 🚀**
