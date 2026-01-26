---
name: motion-canvas
description: Create animated videos and visualizations using Motion Canvas - programmatic video generation with TypeScript
argument-hint: [scene-name, output-format]
context: fork
allowed-tools: Bash(npm*, npx*, node*, ffmpeg*), Read, Write, Edit
---

# Motion Canvas - Animated Video Generation

Motion Canvas is a TypeScript library for creating programmatic videos with code. Perfect for animated git charts, commit visualizations, and data-driven presentations.

## Quick Start

### Installation
```bash
# Create new Motion Canvas project
npm init motion-canvas@latest my-video

# Or add to existing project
npm install @motion-canvas/2d @motion-canvas/core
```

### Basic Structure
```
project/
├── src/
│   └── scenes/
│       └── gitCharts.ts    # Your animation scenes
├── public/                 # Assets (images, fonts)
├── motion.canvas.ts        # Project configuration
└── package.json
```

## Git Chart Examples

### 1. Animated Bar Chart (Commits Over Time)
```typescript
// src/scenes/gitCharts.ts
import { makeScene2D, Circle, Rect, Line, Txt } from '@motion-canvas/2d';
import { createRef, range, easeInOutCubic } from '@motion-canvas/core';

// Your git data
const gitData = [
  { week: '2025-W38', commits: 4, additions: 31333, deletions: 0 },
  { week: '2025-W39', commits: 33, additions: 596417, deletions: 28869 },
  { week: '2025-W40', commits: 40, additions: 211703, deletions: 17026 },
  // ... more data
];

export default makeScene2D(function* (view) {
  const bars = createRef<Rect[]>();
  const labels = createRef<Txt[]>();

  // Create bars
  view.add(
    <Rect layout direction="row" gap={20} ref={bars}>
      {gitData.map((data, i) => (
        <Rect
          width={40}
          height={data.commits * 2}  // Scale height
          fill="#7dd3fc"
          radius={4}
        >
          <Txt
            text={data.week}
            y={50}
            fontSize={16}
            fill="#fff"
          />
        </Rect>
      ))}
    </Rect>
  );

  // Animate bars growing
  yield* all(
    ...gitData.map((data, i) =>
      bars()[i].height(data.commits * 2, 1, easeInOutCubic)
    )
  );
});
```

### 2. Timeline Visualization
```typescript
import { makeScene2D, Circle, Line, Txt } from '@motion-canvas/2d';
import { all, waitFor, chain } from '@motion-canvas/core';

export default makeScene2D(function* (view) {
  // Create timeline line
  const timeline = createRef<Line>();
  view.add(
    <Line
      ref={timeline}
      points={[[-400, 0], [400, 0]]}
      stroke="#666"
      lineWidth={2}
    />
  );

  // Animate commits appearing along timeline
  for (const [i, week] of gitData.entries()) {
    const x = -400 + (i * 40);

    view.add(
      <Circle
        x={x}
        size={week.commits}
        fill="#7dd3fc"
        opacity={0}
      />
    );

    yield* waitFor(0.2);  // Stagger appearance
  }

  yield* waitFor(2);  // Hold final frame
});
```

### 3. Growth Animation (LOC Over Time)
```typescript
export default makeScene2D(function* (view) {
  const currentLoc = createRef<Txt>();

  view.add(
    <>
      <Txt
        ref={currentLoc}
        text="0 LOC"
        fontSize={64}
        fill="#00ff88"
        y={-200}
      />
    </>
  );

  let total = 0;
  for (const week of gitData) {
    total += week.additions - week.deletions;

    // Animate counter
    yield* currentLoc().text(`${total.toLocaleString()} LOC`, 0.5);

    // Add visual marker
    view.add(
      <Rect
        width={5}
        height={Math.abs(week.additions - week.deletions) / 1000}
        x={-400 + gitData.indexOf(week) * 40}
        y={week.additions > week.deletions ? 0 : 100}
        fill={week.additions > week.deletions ? '#00ff88' : '#ff6384'}
      />
    );

    yield* waitFor(0.3);
  }
});
```

## Common Animations

### Fade In
```typescript
yield* rect().opacity(1, 1);
```

### Slide In
```typescript
yield* rect().position([100, 0], 1, easeInOutCubic);
```

### Scale Up
```typescript
yield* circle().scale(2, 1.5);
```

### Color Transition
```typescript
yield* rect().fill('#00ff88', 1);
```

### Rotate
```typescript
yield* rect().rotation(360, 2);
```

## Exporting Video

### Render Settings (motion.canvas.ts)
```typescript
export default {
  scenes: ['./src/scenes/*'],
  projection: 'orthographic',
  size: { x: 1920, y: 1080 },  // 1080p
  fps: 60,
  duration: 10,  // seconds
};
```

### Render Commands
```bash
# Render all scenes
npm run build

# Render specific scene
npx motion-canvas render src/scenes/gitCharts.ts

# Preview in browser
npm run serve

# Export to MP4 (requires ffmpeg)
ffmpeg -r 60 -i frames/%04d.png -c:v libx264 -pix_fmt yuv420p output.mp4
```

## Git Data Integration

### Load from JSON
```typescript
import gitData from './git-activity.json' assert { type: 'json' };

export default makeScene2D(function* (view) {
  for (const week of gitData.weeks) {
    // Use data
  }
});
```

### Fetch from API
```typescript
const gitStats = await fetch('https://api.github.com/users/fabiantax/events')
  .then(r => r.json());

// Create animation from live data
```

### Export from git-activity CLI
```bash
# Generate JSON
git-activity --github-stats --since 2025-11-01 --group-by week --json git-data.json

# Import in Motion Canvas scene
import gitData from './git-data.json' assert { type: 'json' };
```

## Best Practices for Git Charts

1. **Keep animations simple** - 0.5-2 seconds per transition
2. **Use consistent colors** - Match your brand theme
3. **Add labels** - Week numbers, commit counts, LOC values
4. **Stagger elements** - Don't animate everything at once
5. **Hold final frames** - Give time to read the data

## Advanced Features

### Layouts
```typescript
import { Layout } from '@motion-canvas/2d';

<Layout direction="column" gap={20}>
  <Txt text="Git Activity" fontSize={48} />
  <Rect width={800} height={400} />
</Layout>
```

### Grid Layouts
```typescript
<Layout gap={20}>
  {gitData.map(d => (
    <Rect width={100} height={d.commits} />
  ))}
</Layout>
```

### Custom Easing
```typescript
import { tween, map } from '@motion-canvas/core';

const customEasing = t => t * t * (3 - 2 * t);  // Smoothstep
yield* rect().scale(2, 1, customEasing);
```

### Audio Sync
```typescript
import { Audio, beginSlide } from '@motion-canvas/2d';

const audio = Audio.import('./soundtrack.mp3');
yield* all(
  audio.play(),
  animateBars()
);
```

## Resources

- **Documentation**: https://motion-canvas.dev
- **Examples**: https://github.com/motion-canvas/motion-canvas
- **Community**: https://discord.gg/X7WvRunu

## Integration with git-activity-dashboard

1. Export activity data:
```bash
cd git_activity_dashboard
cargo run -- --github-stats --since 2025-11-01 --group-by week --json activity.json
```

2. Copy to Motion Canvas project:
```bash
cp activity.json ../my-video/src/data/git-activity.json
```

3. Import and animate:
```typescript
import data from './data/git-activity.json' assert { type: 'json' };
```

4. Render and share:
```bash
npm run build
# Upload MP4 to LinkedIn, GitHub, or portfolio
```

## Export Formats

- **MP4** - Best for sharing, LinkedIn, presentations
- **WebM** - Web optimization
- **GIF** - Quick previews (lower quality)
- **PNG sequence** - Max quality, post-process in After Effects

## Tips for LinkedIn Videos

- **Duration**: 15-60 seconds (short is better)
- **Resolution**: 1920x1080 or 1080x1920 (vertical)
- **Aspect ratio**: 16:9 (landscape) or 9:16 (mobile)
- **Text size**: At least 48px for readability
- **Captions**: Add text overlays for context
