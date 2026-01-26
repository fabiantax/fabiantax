# User Stories: Video Generation Feature

## Epic: Video Generation from Git Activity Data

### User Story 1: Console Output Export
**As a** developer using git-activity-dashboard
**I want to** save console output to a file
**So that** I can document and share activity summaries

**Acceptance Criteria:**
- [ ] CLI accepts `--output <file>` flag
- [ ] Saves all console output to specified file
- [ ] Works with all existing modes (--github-stats, --group-by, etc.)
- [ ] File includes repository list, activity breakdown table, and visual bars

**Implementation:**
- Added `--output` flag to CLI arguments
- Redirects stdout to file using libc dup2 (Unix)
- Preserves all console formatting

---

### User Story 2: Animated Bar Chart Video Generation
**As a** developer
**I want to** generate animated MP4 videos from my git activity
**So that** I can share visualizations on LinkedIn and portfolios

**Acceptance Criteria:**
- [ ] Generates 1920x1080 MP4 at 60fps
- [ ] Shows weekly commit activity with animated bars
- [ ] Bars grow from 0 to full height (visible growth animation)
- [ ] Staggered animation: bars appear one-by-one from left to right
- [ ] Duration: ~6 seconds, file size < 150KB
- [ ] Professional dark theme (#0a0a0f background, cyan accents)

**Implementation:**
- `examples/videos/generate-frames.js` - Node.js script using canvas2video
- Configurable animation (duration, easing, colors)
- FFmpeg integration for MP4 encoding
- Example data: November 2025 - January 2026 (20 weeks)

---

### User Story 3: Multiple Visualization Options
**As a** content creator
**I want** different ways to visualize my git activity
**So that** I can choose the best format for different platforms

**Acceptance Criteria:**
- [ ] Interactive web charts (web/barcharts.html)
  - Weekly/monthly toggle
  - Moving averages
  - Drag & drop JSON data
  - Real-time chart updates
- [ ] Static image frames (for manual editing)
- [ ] Animated MP4 video
- [ ] Console output (text file)

**Implementation:**
- `web/barcharts.html` - Interactive Chart.js visualization
- `examples/videos/` - Video generation tools
- `--output` flag - Save console output

---

### User Story 4: Animation Customization
**As** a developer
**I want** to customize the animation style
**So that** I can match my brand or preference

**Acceptance Criteria:**
- [ ] Configurable bar width
- [ ] Configurable animation duration
- [ ] Multiple easing functions (linear, ease-out, bounce)
- [ ] Staggered, sequential, or simultaneous growth
- [ ] Custom color schemes
- [ ] Animation guide with examples

**Implementation:**
- `examples/videos/ANIMATION_GUIDE.md` - Comprehensive guide
- `examples/videos/QUICK_EXAMPLES.md` - Copy-paste examples
- Well-documented code with clear variable names

---

### User Story 5: Documentation & Examples
**As** a new user
**I want** clear documentation on how to generate videos
**So that** I can get started quickly

**Acceptance Criteria:**
- [ ] README in examples/videos/ with quick start
- [ ] Animation guide explaining all options
- [ ] Quick examples for common patterns
- [ ] Code comments explaining the animation logic
- [ ] Troubleshooting guide for common issues

**Implementation:**
- `examples/videos/README.md`
- `examples/videos/ANIMATION_GUIDE.md`
- `examples/videos/QUICK_EXAMPLES.md`
- Inline code comments
- Verification scripts (debug-frame0.js, verify-staggered.js)

---

## Technical Implementation Details

### Added Dependencies:
```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
libc = "0.2"  # For stdout redirection (--output flag)
```

### CLI Enhancement:
- **File**: `src/bin/cli.rs`
- **Change**: Added `--output <FILE>` argument
- **Purpose**: Save console output to file

### Video Generator:
- **File**: `examples/videos/generate-frames.js`
- **Features**:
  - Staggered bar growth (12 frames per bar)
  - Ease-out quadratic smoothing
  - Week and year labels
  - Commit counts
  - Summary statistics
  - Frame progress bar

### Web Visualization:
- **File**: `web/barcharts.html`
- **Features**:
  - Interactive Chart.js charts
  - Weekly/monthly toggle
  - 4-period moving average
  - Drag & drop JSON import
  - Real-time preview

### Example Data:
- **File**: `examples/videos/activity-nov2025-jan2026.json`
- **Content**: User's actual git activity (Nov 2025 - Jan 2026)

---

## Definition of Done:

- [x] `--output` flag works and saves console output
- [x] Video generator creates correct staggered animation
- [x] Frame 0 shows empty chart (0 bars visible)
- [x] Frames progress correctly (0 → 10 → 20 bars)
- [x] Final MP4 is < 150KB and 6 seconds
- [ ] Web charts interactive and working
- [ ] Documentation is complete and clear
- [ ] All examples tested and verified
