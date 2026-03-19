# Eid Mubarak! 🌙 ✨

A CLI tool to celebrate Eid Mubarak with stunning ASCII video art directly in your terminal!

## 🚀 Installation

### 1. Install Dependencies
This project relies on `ffmpeg` to process video frames. You need to install the required system packages before installing the tool.

#### Linux (Debian / Ubuntu)
```bash
sudo apt update
sudo apt install ffmpeg libavcodec-dev libavformat-dev libavutil-dev libswscale-dev libavdevice-dev pkg-config clang
```

#### Linux (Arch)
```bash
sudo pacman -S ffmpeg clang pkgconf
```

#### macOS
```bash
brew install ffmpeg pkg-config
```

### 2. Install the Package
You can easily install the tool using **cargo**:

```bash
cargo install Eid-Mubarak
```

## 🧠 How it Works

This tool creates a beautiful terminal-based video player using Rust and `ffmpeg`. Here is how it works under the hood:

1. **Video Decoding**: It reads a video file using the `ffmpeg-next` crate and decodes the video streams frame by frame.
2. **Resizing & Scaling**: Each frame is scaled down to a manageable terminal size (50x20) and converted to RGB pixel format using bilinear scaling.
3. **ASCII Mapping**: For each pixel, the brightness is calculated. Based on the brightness level, the pixel is mapped to a specific ASCII character from a predefined density scale.
4. **Terminal Rendering**: The ASCII characters are printed to the terminal with their original colors using `crossterm`.
5. **Overlay Text**: Beautiful Eid greetings in English (ASCII Art) and Arabic are superimposed onto the video output at specific coordinates.
6. **Frame Synchronization**: The tool calculates the delay between frames to match the original framerate of the video, ensuring smooth playback.

---
*كل عام وأنتم بخير وصحة وسعادة وعافية - تقبل الله منا ومنكم صالح الأعمال*
