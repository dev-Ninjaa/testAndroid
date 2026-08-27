# Icons Directory

Place your app icons here with the following names:
- 32x32.png
- 128x128.png
- 128x128@2x.png
- icon.icns (for macOS)
- icon.ico (for Windows)

You can generate these icons from a single source image using:
```bash
cargo tauri icon path/to/your-icon.png
```

For now, Tauri will use default icons if these are not present.
