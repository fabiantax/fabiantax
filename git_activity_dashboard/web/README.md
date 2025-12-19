# Git Activity Constellation

Pure canvas visualization of your git activity - no dependencies, no iframes, just drop it in.

## Quick Start

```html
<div id="my-constellation" style="width: 800px; height: 500px;"></div>
<script src="git-constellation.js"></script>
<script>
    GitConstellation.init('#my-constellation', 'https://yoursite.com/activity.json');
</script>
```

That's it. One script, one div, done.

## Generate Your Data

```bash
# Single repo
git-activity -r /path/to/repo --json activity.json

# Multiple repos
git-activity -s /path/to/projects --json activity.json
```

## API

### Simple Init
```js
// Load from URL
GitConstellation.init('#container', 'activity.json');

// With options
GitConstellation.init('#container', 'activity.json', {
    width: 800,
    height: 500,
    animate: true,      // Enable animations (default: true)
    showLabels: true,   // Show node labels (default: true)
    showStats: true     // Show stats panel (default: true)
});
```

### Manual Control
```js
// Create instance
const viz = GitConstellation.create('#container', { width: 800, height: 500 });

// Load data
viz.loadFromUrl('activity.json');
// or
viz.loadData({ summary: {...}, repositories: [...] });

// Resize
viz.resize(1000, 600);

// Cleanup
viz.destroy();
```

## Features

- **Zero dependencies** - Pure canvas, no external libs
- **Animated** - Subtle floating animation and glow effects
- **Interactive** - Hover for node details
- **Responsive** - Call `resize()` on window resize
- **Fast** - GPU-accelerated canvas rendering

## Node Types

| Color | Type |
|-------|------|
| Pink | Repository |
| Cyan | Programming Language |
| Purple | Contribution Type (code/tests/docs) |
| Green | File Extension |

Node size = activity level. Edges connect repos to their languages, contribution types, and file extensions.

## Embed Examples

### Portfolio Site
```html
<section class="git-activity">
    <h2>My Coding Activity</h2>
    <div id="constellation" style="width:100%;height:400px;"></div>
</section>
<script src="git-constellation.js"></script>
<script>
    GitConstellation.init('#constellation', '/data/activity.json', {
        showStats: true
    });
</script>
```

### Auto-refresh
```js
setInterval(async () => {
    const res = await fetch('/data/activity.json');
    const data = await res.json();
    constellation.loadData(data);
}, 60000); // Refresh every minute
```

## Files

- `git-constellation.js` - The visualization (drop this in your site)
- `index.html` - Demo page
- `sample-data.json` - Example data
