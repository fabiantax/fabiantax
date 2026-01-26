# Animation Customization Guide

## Quick Overview

The animation is controlled by parameters in `generate-frames.js`. Here's how to customize every aspect:

## 1. Duration & Speed

**Current Settings (Line 13-14):**
```javascript
const fps = 60;           // Frames per second
const duration = 6;       // Total video length in seconds
const totalFrames = fps * duration;  // = 360 frames
```

**Options:**

| Change | Effect |
|--------|--------|
| `duration = 3` | Faster video (3 seconds) |
| `duration = 10` | Slower, more detailed (10 seconds) |
| `fps = 30` | Lower frame rate (smaller file) |
| `fps = 120` | Smoother animation (larger file) |

**Example: Quick 15-second video for LinkedIn**
```javascript
const fps = 60;
const duration = 15;  // 15 seconds
```

---

## 2. Animation Timing

**Current (Line 53):**
```javascript
const barsGrowEnd = Math.floor(totalFrames * 0.6);  // Bars grow for 60% of video
```

**Animation Timeline:**
```
Frame 0-216 (60%): Bars growing from 0 to full height
Frame 216-270: Labels fade in
Frame 270-360: Statistics fade in
```

**Custom Examples:**

**Fast grow, long hold:**
```javascript
const barsGrowEnd = Math.floor(totalFrames * 0.3);  // Bars grow in 30% of time
```

**Slow grow, short hold:**
```javascript
const barsGrowEnd = Math.floor(totalFrames * 0.8);  // Bars grow in 80% of time
```

---

## 3. Easing Functions (Animation Curve)

**Current (Line 72-73):**
```javascript
const easeOut = t => 1 - Math.pow(1 - t, 3);
const easedProgress = easeOut(growProgress);
```

**Easing Options:**

```javascript
// Linear (no easing)
const linear = t => t;

// Ease Out (starts fast, slows down) - CURRENT
const easeOut = t => 1 - Math.pow(1 - t, 3);

// Ease In (starts slow, speeds up)
const easeIn = t => t * t * t;

// Ease In-Out (slow start, fast middle, slow end)
const easeInOut = t => t < 0.5
  ? 4 * t * t * t
  : 1 - Math.pow(-2 * t + 2, 3) / 2;

// Bounce (bouncy effect)
const bounce = t => {
  const n1 = 7.5625;
  const d1 = 2.75;
  if (t < 1 / d1) return n1 * t * t;
  if (t < 2 / d1) return n1 * (t -= 1.5 / d1) * t + 0.75;
  if (t < 2.5 / d1) return n1 * (t -= 2.25 / d1) * t + 0.9375;
  return n1 * (t -= 2.625 / d1) * t + 0.984375;
};
```

**To use:**
```javascript
const easedProgress = bounce(growProgress);  // Change easeOut to bounce
```

---

## 4. Colors

**Background (Line 56-59):**
```javascript
const gradient = ctx.createLinearGradient(0, 0, width, height);
gradient.addColorStop(0, '#0a0a0f');      // Top color
gradient.addColorStop(1, '#16162a');      // Bottom color
```

**Bar Colors (Line 87-93):**
```javascript
const intensity = Math.min(1, week.commits / 150);
const r = Math.floor(56 + intensity * 69);   // Red channel
const g = Math.floor(189 + intensity * 22);  // Green channel
const b = Math.floor(248 - intensity * 50);  // Blue channel

ctx.fillStyle = `rgba(${r}, ${g}, ${b}, 0.8)`;
```

**Custom Color Schemes:**

**Purple gradient:**
```javascript
gradient.addColorStop(0, '#1a1a2e');
gradient.addColorStop(1, '#16213e');
```

**Green bars:**
```javascript
const intensity = Math.min(1, week.commits / 150);
const r = Math.floor(0 + intensity * 50);
const g = Math.floor(255);
const b = Math.floor(136 + intensity * 50);
ctx.fillStyle = `rgba(${r}, ${g}, ${b}, 0.8)`;
```

**Fixed color (no gradient):**
```javascript
ctx.fillStyle = '#7dd3fc';  // Solid cyan
```

---

## 5. Text & Fonts

**Title (Line 62-65):**
```javascript
ctx.fillStyle = '#7dd3fc';              // Color
ctx.font = 'bold 56px Arial, sans-serif';  // Size & family
ctx.textAlign = 'center';               // Alignment
ctx.fillText('🚀 Git Activity Dashboard', width / 2, 80);  // Text & position
```

**Custom Examples:**

**Larger, different font:**
```javascript
ctx.font = 'bold 72px "Courier New", monospace';
```

**Different color:**
```javascript
ctx.fillStyle = '#00ff88';  // Bright green
```

---

## 6. Bar Dimensions

**Current (Line 50-51):**
```javascript
const barWidth = 50;
const gap = (chartWidth - barWidth * gitData.length) / (gitData.length + 1);
```

**Custom Options:**

**Thicker bars:**
```javascript
const barWidth = 80;
```

**Thinner bars:**
```javascript
const barWidth = 30;
```

**Custom spacing:**
```javascript
const barWidth = 50;
const gap = 20;  // Fixed 20px gap
```

---

## 7. Animation Effects

### Staggered Bar Growth

**Current:** All bars grow together

**Staggered (one by one):**
```javascript
gitData.forEach((week, i) => {
  const staggerDelay = i * 10;  // 10 frame delay per bar
  const adjustedFrame = Math.max(0, frame - staggerDelay);
  const growProgress = Math.min(1, adjustedFrame / barsGrowEnd);
  const easedProgress = easeOut(growProgress);

  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
  const currentHeight = targetHeight * easedProgress;

  // Draw bar...
});
```

### Wave Effect

**Bars grow in a wave:**
```javascript
gitData.forEach((week, i) => {
  const waveOffset = Math.sin(frame * 0.05 + i * 0.5) * 0.2;
  const growProgress = Math.min(1, (frame / barsGrowEnd) + waveOffset);

  // Draw bar...
});
```

### Bounce Effect

**Bars bounce when reaching full height:**
```javascript
gitData.forEach((week, i) => {
  const growProgress = Math.min(1, frame / barsGrowEnd);
  const bounce = growProgress < 1
    ? 1 - Math.pow(1 - growProgress, 3)  // Ease out cubic
    : 1 + Math.sin(growProgress * 10) * 0.05 * (1 - growProgress);  // Small bounce

  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
  const currentHeight = targetHeight * bounce;

  // Draw bar...
});
```

### Glow Effect

**Add glow around bars:**
```javascript
ctx.shadowBlur = 20;
ctx.shadowColor = '#7dd3fc';
ctx.fillRect(x, chartBottom - currentHeight, barWidth, currentHeight);
ctx.shadowBlur = 0;  // Reset for other elements
```

---

## 8. Advanced: Moving Timeline

**Animate bars from left to right:**
```javascript
gitData.forEach((week, i) => {
  const x = chartLeft + gap + i * (barWidth + gap);
  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;

  // Timeline animation (bars appear from left)
  const timelineProgress = (frame - i * 5) / barsGrowEnd;
  const growProgress = Math.max(0, Math.min(1, timelineProgress));
  const easedProgress = easeOut(growProgress);

  const currentHeight = targetHeight * easedProgress;

  // Only draw if timeline has reached this bar
  if (growProgress > 0) {
    ctx.fillRect(x, chartBottom - currentHeight, barWidth, currentHeight);
  }
});
```

---

## 9. Fade Effects

**Fade in title:**
```javascript
const titleAlpha = Math.min(1, frame / 30);  // Fade in over 0.5 seconds
ctx.fillStyle = `rgba(125, 211, 252, ${titleAlpha})`;
ctx.fillText('🚀 Git Activity Dashboard', width / 2, 80);
```

**Pulse effect:**
```javascript
const pulse = 0.7 + Math.sin(frame * 0.1) * 0.3;  // 0.4 to 1.0
ctx.fillStyle = `rgba(125, 211, 252, ${pulse})`;
```

---

## 10. Complete Customization Example

**Bouncy, staggered, glowing bars:**
```javascript
gitData.forEach((week, i) => {
  const staggerDelay = i * 8;
  const adjustedFrame = Math.max(0, frame - staggerDelay);
  const growProgress = Math.min(1, adjustedFrame / 180);

  // Add bounce
  const bounce = growProgress < 1
    ? 1 - Math.pow(1 - growProgress, 2)
    : 1 + Math.sin((growProgress - 1) * Math.PI * 2) * 0.1 * (1 - growProgress);

  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
  const currentHeight = targetHeight * bounce;

  const x = chartLeft + gap + i * (barWidth + gap);

  // Add glow
  ctx.shadowBlur = 30;
  ctx.shadowColor = '#38bdf8';

  ctx.fillStyle = '#38bdf8';
  ctx.fillRect(x, chartBottom - currentHeight, barWidth, currentHeight);

  ctx.shadowBlur = 0;  // Reset
});
```

---

## Quick Customization Commands

```bash
# Edit the file
nano generate-frames.js

# Regenerate frames
node generate-frames.js

# Create video
ffmpeg -r 60 -i frames/frame-%04d.png -c:v libx264 -preset slow -crf 20 -pix_fmt yuv420p my-custom-video.mp4
```

---

## Popular Styles

### LinkedIn Professional (Current)
- Duration: 6 seconds
- Colors: Cyan on dark background
- Animation: Smooth ease-out

### High Energy
```javascript
const duration = 10;
const barWidth = 70;
// Use bounce easing
const easedProgress = bounce(growProgress);
```

### Minimalist
```javascript
const duration = 3;
const barWidth = 40;
ctx.fillStyle = '#ffffff';  // White bars
// No effects
```

### Colorful
```javascript
const colors = ['#ff6b6b', '#feca57', '#48dbfb', '#ff9ff3', '#54a0ff'];
gitData.forEach((week, i) => {
  ctx.fillStyle = colors[i % colors.length];
  // Draw bar...
});
```

---

## Tips

1. **Start small** - Change one thing at a time
2. **Test frequently** - Generate a short video first (3 seconds)
3. **Preview frames** - Open `frames/frame-0180.png` to see the middle frame
4. **Use easing** - Makes animations look professional
5. **Consider file size** - Longer duration = larger file
