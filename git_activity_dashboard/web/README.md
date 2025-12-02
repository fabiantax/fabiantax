# Git Activity Constellation

Interactive visualization of your git activity using Cosmos.gl.

## Features

- **Constellation View**: Full-screen interactive graph visualization
  - Repositories (pink nodes)
  - Programming languages (cyan nodes)
  - Contribution types (purple nodes)
  - File extensions (green nodes)
  - Connections show relationships between nodes

- **Embeddable Widget**: Compact widget for your personal website
  - Key stats overview
  - Contribution breakdown bars
  - Monthly activity sparkline

## Usage

### 1. Generate Your Data

```bash
# From your project directory
git-activity -r /path/to/your/repo --json activity.json

# Or scan multiple repos
git-activity -s /path/to/projects --json activity.json
```

### 2. View the Constellation

Open `index.html` in a browser and either:
- Paste your JSON data
- Drag & drop your JSON file
- Or load via URL parameter: `index.html?data=https://yoursite.com/activity.json`

### 3. Embed on Your Website

#### Option A: Widget Component

```html
<iframe
    src="embed.html?data=https://yoursite.com/activity.json"
    width="400"
    height="600"
    frameborder="0"
    style="border-radius: 16px;">
</iframe>
```

#### Option B: Direct Integration

```html
<div class="git-widget"
     id="git-widget"
     data-url="https://yoursite.com/activity.json">
</div>
<script src="widget.js"></script>
```

#### Option C: JavaScript API

```html
<script src="constellation.js"></script>
<script>
    fetch('/activity.json')
        .then(r => r.json())
        .then(data => GitConstellation.loadFromJson(data));
</script>
```

## Hosting on GitHub Pages

1. Copy the `web/` folder to your repo
2. Upload your `activity.json`
3. Enable GitHub Pages in repo settings
4. Access at: `https://username.github.io/repo/web/index.html?data=activity.json`

## Auto-Update with GitHub Actions

```yaml
# .github/workflows/update-activity.yml
name: Update Git Activity

on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
  push:
    branches: [main]

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Build CLI
        run: cargo build --release --features cli
        working-directory: git_activity_dashboard

      - name: Generate Activity Data
        run: |
          ./git_activity_dashboard/target/release/git-activity \
            -s . --json web/activity.json -q

      - name: Commit & Push
        run: |
          git config user.name "GitHub Action"
          git config user.email "action@github.com"
          git add web/activity.json
          git commit -m "Update git activity data" || true
          git push
```

## Customization

Edit the CSS variables in the HTML files to match your site's theme:

```css
:root {
    --bg-color: #0a0a0f;
    --text-color: #e0e0e0;
    --accent-color: #7dd3fc;
    --repo-color: #f472b6;
    --lang-color: #7dd3fc;
    --contrib-color: #a78bfa;
    --ext-color: #34d399;
}
```

## Files

- `index.html` - Full constellation visualization
- `constellation.js` - Cosmos.gl visualization logic
- `embed.html` - Embeddable widget
- `sample-data.json` - Example data file
