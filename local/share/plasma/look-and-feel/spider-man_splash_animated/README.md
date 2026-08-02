# 🕷️ Spider-Man Splash Screen
*A Spider-Man themed KDE Plasma 6 splash screen featuring the classic red & blue color palette. Fork of Styrup Jazz Animated Splash.*

![Preview](contents/previews/splash.png)

---

## ✨ Features
- **Spider-Man classic theme** with iconic red & blue colors
- **Pure black background** for dramatic contrast
- **Smooth animations** inherited from original jazz theme
- **Pixel-perfect Loading** text and animated progress bar
- **Bitmap-style font** Pixel Emulator
- **Web-slinging vibes** for your daily KDE startup

---

## 🚀 Installation

### Method 1: Manual Installation
1. **Download** the latest release or clone this repository
2. **Copy the package** folder into your Plasma look-and-feel directory:
   ```bash
   cp -r spider-man_splash_animated ~/.local/share/plasma/look-and-feel/
   ```
3. **Activate** in System Settings > Appearance > Splash Screen > **Spider-Splash**
4. **Log out & back in** *(or restart Plasma)* to see your friendly neighborhood splash screen

### Method 2: KDE Store
Coming soon on store.kde.org

#### 🎨 Customization
The Spider-Man color palette is defined in the theme properties:
  ```qml
  qmlproperty var theme: ({
      primary: "#DC143C",      // Spider-Man red
      secondary: "#0047AB",    // Spider-Man blue
      background: "#000000",   // Pure black
      // ... more colors
  })
  ```

#### 📄 License
This package is **dual-licensed**:

- **QML code & helper scripts**
GNU GPL v3 (or later)
Full text
- **Images & fonts**
CC BY-SA 4.0 International
Full text

See **LICENSE** for full details.

#### 🙏 Credits

- **Original Code & Concept**: Jim Sines - raystanza@raystanza.uk
- **Spider-Man Fork**: SAWKIT - sawkit17@icloud.com
- **Original Project**: Styrup Jazz Animated Splash


#### 🐛 Issues & Contributing
Found a bug or want to contribute? Feel free to:

- Open an issue on GitHub
- Submit a pull request
-- Suggest new Spider-Man variants (Miles Morales, Spider-Gwen, etc.)


With great power comes great splash screens! 🕸️
