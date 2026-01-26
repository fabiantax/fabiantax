# Quick Animation Examples

Copy these into `generate-frames.js` to try different styles.

## Example 1: Fast 3-Second Video
```javascript
// Line 14
const duration = 3;  // Fast!

// Line 53
const barsGrowEnd = Math.floor(totalFrames * 0.8);  // Slower grow
```

## Example 2: Bouncy Animation
```javascript
// Replace lines 72-73 with:
const bounce = t => {
  const n1 = 7.5625, d1 = 2.75;
  if (t < 1/d1) return n1*t*t;
  if (t < 2/d1) return n1*(t-=1.5/d1)*t+.75;
  return n1*(t-=2.625/d1)*t+.984375;
};
const easedProgress = bounce(growProgress);
```

## Example 3: Staggered Bars (One by One)
```javascript
// Replace the forEach loop (line 82) with:
gitData.forEach((week, i) => {
  const staggerDelay = i * 8;  // 8 frames delay per bar
  const adjustedFrame = Math.max(0, frame - staggerDelay);
  const growProgress = Math.min(1, adjustedFrame / 180);
  const easedProgress = easeOut(growProgress);
  
  const x = chartLeft + gap + i * (barWidth + gap);
  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
  const currentHeight = targetHeight * easedProgress;
  
  // ... rest of drawing code
```

## Example 4: Glowing Bars
```javascript
// Before line 82, add:
ctx.shadowBlur = 30;
ctx.shadowColor = '#38bdf8';

// After line 119, reset:
ctx.shadowBlur = 0;
```

## Example 5: Rainbow Colors
```javascript
// Replace line 88-90 with:
const colors = ['#ff6b6b', '#feca57', '#48dbfb', '#ff9ff3', '#54a0ff', '#5f27cd'];
const color = colors[i % colors.length];
ctx.fillStyle = color;
```

## Example 6: Thicker Bars
```javascript
// Line 50
const barWidth = 80;  // Was 50
```

---

## How to Use:

1. Edit generate-frames.js
2. Save
3. Run: `node generate-frames.js`
4. Encode: `ffmpeg -r 60 -i frames/frame-%04d.png -c:v libx264 output.mp4`

## Bar Growth Styles

### Style 1: Staggered Wave (Left to Right)
```javascript
// Replace the forEach loop with:
let lastYear = null;

gitData.forEach((week, i) => {
  const x = chartLeft + gap + i * (barWidth + gap);

  // Stagger: each bar starts 8 frames after the previous
  const staggerDelay = i * 8;
  const adjustedFrame = Math.max(0, frame - staggerDelay);
  const growProgress = Math.min(1, adjustedFrame / 180);
  const easedProgress = easeOut(growProgress);

  const [year, weekNum] = week.period.split('-');
  const showYear = year !== lastYear;
  if (showYear) lastYear = year;

  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
  const currentHeight = targetHeight * easedProgress;

  // ... rest of drawing code same as before
```

### Style 2: Sequential (One completes, then next)
```javascript
// Replace line 53 with:
const barsGrowEnd = Math.floor(totalFrames * 0.8);
const framesPerBar = barsGrowEnd / gitData.length;

// In forEach loop:
const barStartFrame = i * framesPerBar;
const barEndFrame = barStartFrame + framesPerBar;
const growProgress = Math.min(1, Math.max(0, (frame - barStartFrame) / framesPerBar));
const easedProgress = easeOut(growProgress);
```

### Style 3: Slower Growth (More visible)
```javascript
// Line 53: Change growth duration
const barsGrowEnd = Math.floor(totalFrames * 0.85);  // Was 0.6, now 85% of video
```

### Style 4: Wave Effect (Sine wave)
```javascript
// In forEach loop, replace currentHeight calculation:
const waveOffset = Math.sin(frame * 0.03 + i * 0.3) * 0.15;
const growProgress = Math.min(1, frame / barsGrowEnd);
const currentHeight = targetHeight * (growProgress + waveOffset);
```
