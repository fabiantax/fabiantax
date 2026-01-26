# Git Activity Video Generator

Quick prototype for generating animated MP4 videos from git-activity-dashboard data.

## Quick Start

```bash
# 1. Install dependencies
npm install

# 2. Generate frames from your activity data
node generate-frames.js activity.json

# 3. Convert frames to video (requires ffmpeg)
ffmpeg -r 60 -i frames/frame-%04d.png -c:v libx264 -preset slow -crf 20 -pix_fmt yuv420p git-activity.mp4
```

## Example: Using Your Git Activity Data

```bash
# From git_activity_dashboard directory:
export GITHUB_TOKEN=$(gh auth token)
cargo run -- --github-stats --since 2025-11-01 --group-by week --json /tmp/git-video-generator/activity.json

# From /tmp/git-video-generator:
node generate-frames.js
# Creates 360 frames (6 seconds at 60fps)

# Create video:
ffmpeg -r 60 -i frames/frame-%04d.png -c:v libx264 -preset slow -crf 20 -pix_fmt yuv420p git-activity.mp4
```

## Output

- **Resolution**: 1920x1080 (Full HD)
- **Frame rate**: 60 fps
- **Duration**: 6 seconds
- **Format**: MP4 (H.264)
- **Size**: ~2-5 MB

## Features

- ✅ Animated bar chart (bars grow with easing)
- ✅ Week labels (rotated -45°)
- ✅ Commit counts (fade in)
- ✅ Summary statistics (fade in)
- ✅ Progress bar
- ✅ Dark theme matching git-activity-dashboard aesthetic

## Customization

Edit `generate-frames.js` to customize:

- **Duration**: Change `const duration = 6;` (line 12)
- **Colors**: Modify RGB values in bar rendering loop
- **Title**: Change `ctx.fillText('🚀 Git Activity Dashboard', ...)` (line 52)
- **Animation curve**: Change easing function `easeOut` (line 36)

## Requirements

- Node.js (for frame generation)
- ffmpeg (for video encoding)
- git-activity-dashboard (for data export)

## Installing ffmpeg

```bash
# Ubuntu/Debian
sudo apt install ffmpeg

# macOS
brew install ffmpeg

# Windows
# Download from: https://ffmpeg.org/download.html
```

## Files

- `generate-frames.js` - Generate PNG frames from JSON data
- `activity.json` - Sample git activity data (your Nov 2025 - Jan 2026)
- `frames/` - Output directory (auto-created)

## Next Steps

For more advanced video generation:

1. **Remotion** - React-based, best quality: `.claude/skills/video-charts`
2. **Motion Canvas** - TypeScript animations: `/motion-canvas`
3. **canvas2video** - Direct JSON to MP4: See video-charts skill

## Troubleshooting

**"Cannot find module 'canvas'"**
```bash
npm install canvas
```

**"ffmpeg: not found"**
```bash
sudo apt install ffmpeg  # Linux
brew install ffmpeg      # macOS
```

**Frames not generating**
- Check JSON format matches expected structure
- Verify `activity.json` exists and is valid JSON
- Check console for error messages

## License

MIT - Part of git-activity-dashboard project
