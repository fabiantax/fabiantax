---
name: video-charts
description: Generate animated videos from git activity data using canvas2video, Remotion, Puppeteer, or Motion Canvas
argument-hint: [method, input-json, output-mp4]
allowed-tools: Bash(npm*, npx*, node*, ffmpeg*, puppeteer*), Read, Write
---

# Video Generation for Git Charts

Create professional animated videos from git activity data for LinkedIn, portfolios, or presentations.

## Available Methods

### 1. canvas2video (Recommended for Git Stats)
**Best for:** Quick data-driven charts from JSON

```bash
npm install canvas2video
```

```javascript
// generate-video.js
const { Canvas2Video } = require('canvas2video');
const { createCanvas } = require('canvas');

const gitData = [
  { week: '2025-W38', commits: 4, loc: 31333 },
  { week: '2025-W39', commits: 33, loc: 567548 },
  // ... more data
];

const canvas = createCanvas(1920, 1080);
const ctx = canvas.getContext('2d');

const video = new Canvas2Video(canvas, {
  fps: 60,
  bitrate: 5000,
});

// Animate bars growing
for (let frame = 0; frame < 300; frame++) {
  ctx.fillStyle = '#0a0a0f';
  ctx.fillRect(0, 0, 1920, 1080);

  gitData.forEach((data, i) => {
    const progress = Math.min(1, frame / 200);
    const height = data.commits * 5 * progress;
    const x = 100 + i * 80;

    ctx.fillStyle = '#7dd3fc';
    ctx.fillRect(x, 900 - height, 50, height);
  });

  video.addFrame();
}

video.save('git-activity.mp4');
```

**Run:**
```bash
node generate-video.js
```

---

### 2. Remotion (React-based)
**Best for:** React developers, complex compositions

```bash
npm init video@latest my-git-video
cd my-git-video
npm install
```

```tsx
// src/GitChart.tsx
import { useCurrentFrame, useVideoConfig, interpolate } from 'remotion';

export const GitChart = () => {
  const frame = useCurrentFrame();
  const { fps, durationInFrames } = useVideoConfig();

  const gitData = [
    { week: 'W38', commits: 4 },
    { week: 'W39', commits: 33 },
    { week: 'W40', commits: 40 },
  ];

  return (
    <div style={{
      backgroundColor: '#0a0a0f',
      color: 'white',
      fontSize: '48px',
      display: 'flex',
      gap: '20px',
      padding: '100px'
    }}>
      {gitData.map((data, i) => {
        const scale = interpolate(
          frame,
          [i * 10, i * 10 + 30],
          [0, 1],
          { extrapolateRight: 'clamp' }
        );

        return (
          <div key={i}>
            <div style={{ height: data.commits * 10 * scale, background: '#7dd3fc' }} />
            <div>{data.week}</div>
            <div>{data.commits} commits</div>
          </div>
        );
      })}
    </div>
  );
};
```

**Render:**
```bash
npx remotion render src/Root.tsx GitChart output.mp4
```

---

### 3. Puppeteer/Playwright
**Best for:** Web-first libraries (Chart.js, D3.js)

```bash
npm install puppeteer
```

```javascript
// record-chart.js
const puppeteer = require('puppeteer');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();

  // Load your chart HTML
  await page.goto('file:///path/to/barcharts.html');

  // Wait for animations
  await page.waitForTimeout(2000);

  // Record video using ffmpeg
  const { execSync } = require('child_process');
  execSync(`
    ffmpeg -f image2pipe -vcodec ppm -i - \
      -vcodec libx264 -pix_fmt yuv420p -y output.mp4
  `, { input: await page.screenshot() });

  await browser.close();
})();
```

**Or use screen recording:**
```javascript
// Capture frames
for (let i = 0; i < 300; i++) {
  await page.screenshot({ path: `frames/frame-${i.toString().padStart(4, '0')}.png` });
  await page.waitForTimeout(33); // 30fps
}

// Stitch with ffmpeg
execSync('ffmpeg -r 30 -i frames/frame-%04d.png -c:v libx264 output.mp4');
```

---

### 4. Motion Canvas
**Best for:** Programmatic control, math animations

See `/motion-canvas` skill for detailed examples.

---

## Quick Start with git-activity-dashboard

### Step 1: Export Data
```bash
# Export weekly stats to JSON
cd git_activity_dashboard
export GITHUB_TOKEN=$(gh auth token)
cargo run -- --github-stats --since 2025-11-01 --group-by week --json activity.json

# Export monthly stats
cargo run -- --github-stats --since 2025-11-01 --group-by month --json activity-monthly.json
```

### Step 2: Generate Video

**Option A: canvas2video (Fastest)**
```bash
npm install canvas2video canvas
cat > generate.js << 'EOF'
const { Canvas2Video } = require('canvas2video');
const { createCanvas } = require('canvas');
const fs = require('fs');

const data = JSON.parse(fs.readFileSync('activity.json', 'utf8'));
const weeks = data.weekly_stats || [];

const canvas = createCanvas(1920, 1080);
const ctx = canvas.getContext('2d');
const video = new Canvas2Video(canvas, { fps: 60 });

const maxCommits = Math.max(...weeks.map(w => w.commits));

for (let frame = 0; frame < 360; frame++) {
  // Background
  ctx.fillStyle = '#0a0a0f';
  ctx.fillRect(0, 0, 1920, 1080);

  // Title
  ctx.fillStyle = '#7dd3fc';
  ctx.font = 'bold 48px Arial';
  ctx.fillText('Git Activity: Nov 2025 - Jan 2026', 50, 80);

  // Draw bars
  const barWidth = 60;
  const gap = 30;
  weeks.forEach((week, i) => {
    const progress = Math.min(1, frame / 240);
    const x = 100 + i * (barWidth + gap);
    const targetHeight = (week.commits / maxCommits) * 600;
    const height = targetHeight * progress;

    // Bar
    ctx.fillStyle = '#38bdf8';
    ctx.fillRect(x, 900 - height, barWidth, height);

    // Label
    ctx.fillStyle = '#fff';
    ctx.font = '24px Arial';
    ctx.fillText(week.period, x, 940);

    // Value
    ctx.fillStyle = '#00ff88';
    ctx.fillText(week.commits.toString(), x, 900 - height - 10);
  });

  video.addFrame();
}

video.save('git-activity.mp4');
console.log('✅ Saved to git-activity.mp4');
EOF

node generate.js
```

**Option B: Remotion (Best Quality)**
```bash
npm init video@latest git-video
cd git-video

# Copy activity data
cp ../activity.json src/data.json

# Create chart component (see Remotion example above)
# ...

# Render
npm run build
npx remotion render src/Root.tsx GitChart git-activity.mp4
```

**Option C: Motion Canvas (Most Control)**
```bash
npm init motion-canvas@latest git-animation
cd git-animation

# Copy activity data
cp ../activity.json src/data.json

# Create scene (see /motion-canvas skill)
# ...

# Render
npm run build
ffmpeg -r 60 -i frames/%04d.png -c:v libx264 -pix_fmt yuv420p git-activity.mp4
```

---

## LinkedIn Video Best Practices

### Resolution
- **Landscape**: 1920x1080 (16:9) - Desktop
- **Square**: 1080x1080 (1:1) - Mobile feed
- **Vertical**: 1080x1920 (9:16) - Stories

### Duration
- **Optimal**: 15-30 seconds
- **Maximum**: 10 minutes (but short is better)

### Style
- **Dark background**: #0a0a0f or similar
- **Bright accents**: Cyan (#7dd3fc), Green (#00ff88)
- **Large text**: Minimum 48px for readability
- **Smooth animations**: 60fps, 0.5-2s transitions

### Content
1. **Hook** (0-2s): Title, your name
2. **Data** (2-15s): Main visualization
3. **Insights** (15-25s): Key takeaways
4. **CTA** (25-30s): "Follow for more"

---

## Comparison Table

| Method | Setup Time | Render Speed | Quality | Best For |
|--------|-----------|--------------|---------|----------|
| canvas2video | 5 min | Fast | Good | Data charts, quick exports |
| Remotion | 15 min | Medium | Excellent | React devs, compositions |
| Puppeteer | 10 min | Slow | Good | Web libs (Chart.js, D3) |
| Motion Canvas | 10 min | Fast | Excellent | Code animations, math |

---

## Export Command Reference

### ffmpeg (Manual Stitching)
```bash
# PNG sequence to MP4
ffmpeg -r 60 -i frames/%04d.png -c:v libx264 -pix_fmt yuv420p output.mp4

# Optimize for web
ffmpeg -i input.mp4 -c:v libx264 -crf 28 -preset slow output.mp4

# Add audio
ffmpeg -i video.mp4 -i audio.mp3 -c:v copy -c:a aac output.mp4

# Create GIF (lower quality)
ffmpeg -i input.mp4 -vf "fps=10,scale=480:-1" output.gif
```

---

## Resources

- **canvas2video**: https://github.com/pankod/canvas2video
- **Remotion**: https://www.remotion.dev/
- **Motion Canvas**: https://motion-canvas.dev/
- **Puppeteer**: https://pptr.dev/
- **ffmpeg docs**: https://ffmpeg.org/documentation.html

---

## Example Workflow

```bash
# 1. Get git data
git-activity --github-stats --since 2025-11-01 --json activity.json

# 2. Generate video (choose one method)
node generate.js                    # canvas2video
npx remotion render ...             # Remotion
npm run build && ffmpeg ...         # Motion Canvas

# 3. Optimize for web
ffmpeg -i git-activity.mp4 -c:v libx264 -crf 28 -preset slow final.mp4

# 4. Share
# Upload to LinkedIn, GitHub, or portfolio
```

---

## Tips for Git Activity Videos

1. **Tell a story**: Show growth, highlight milestones
2. **Use consistent colors**: Match your brand
3. **Add context**: Week numbers, dates, totals
4. **Keep it simple**: Don't over-animate
5. **Test on mobile**: Most viewers use phones
