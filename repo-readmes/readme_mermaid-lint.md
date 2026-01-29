# mermaid-lint

Ultra-fast Mermaid diagram linter - **200,000+ diagrams/sec** with sublinear optimizations.

```bash
npx mermaid-lint check docs/
```

## Performance

| Metric | Value |
|--------|-------|
| File skip (SIMD indexOf) | **577ns** for 100KB |
| Trigger check | **167ns** |
| Tokenizer speedup | **7.1x** vs naive |
| Per-diagram processing | **4.9µs** |
| Throughput | **204,538 diagrams/sec** |

## Installation

```bash
# Use directly with npx
npx mermaid-lint check docs/

# Or install globally
npm install -g mermaid-lint
```

## Usage

```bash
# Lint files/directories
mermaid-lint check README.md
mermaid-lint check docs/

# Auto-fix issues
mermaid-lint fix docs/

# Output as JSON (for CI)
mermaid-lint check . --json

# Disable specific rules
mermaid-lint check . --rule use-flowchart:off
```

## Rules

### Errors (break rendering)

| Rule | Description |
|------|-------------|
| `no-br-tags` | HTML `<br/>` breaks Mermaid rendering |
| `no-html-tags` | HTML tags not supported in labels |
| `use-flowchart` | Use `flowchart` instead of deprecated `graph` |
| `arrow-missing-source` | Arrow with no source node (`--> B`) |
| `arrow-missing-target` | Arrow with no target node (`A -->`) |
| `mismatched-bracket` | Mismatched brackets like `[)` |
| `unclosed-bracket` | Unclosed `[`, `(`, or `{` |
| `unclosed-block` | Unclosed `subgraph` or `loop` |

### Warnings

| Rule | Description |
|------|-------------|
| `no-self-loop` | Self-referencing connection (`A --> A`) |
| `duplicate-connection` | Same connection appears multiple times¹ |
| `consistent-arrow-style` | Mixed arrow styles (normal, thick, dotted) |
| `undefined-node` | Node referenced but never defined |
| `duplicate-node-id` | Same node ID defined twice |
| `quote-special-chars` | Unquoted `$#@&%` in labels |
| `no-end-node-id` | Reserved word `end` used as node ID |

¹ `duplicate-connection` compares source→target only, ignoring edge labels. Connections in different subgraphs are still flagged. Disable with `--rule duplicate-connection:off` if needed.

## Configuration

Create `.mermaidlintrc` in your project root:

```json
{
  "rules": {
    "use-flowchart": "warn",
    "quote-special-chars": "off"
  },
  "maxIssues": 100
}
```

Or add to `package.json`:

```json
{
  "mermaidlint": {
    "rules": {
      "use-flowchart": "warn"
    }
  }
}
```

## API

```javascript
const { lintMarkdown, lintDiagram } = require('mermaid-lint');

// Lint markdown file content
const result = lintMarkdown(markdownContent);
console.log(result.issues); // Array of issues
console.log(result.stats);  // { blockCount, errorCount, warnCount }

// Lint single diagram
const { issues } = lintDiagram('flowchart LR\n  A --> B');
```

## Why So Fast?

1. **SIMD File Skip** - V8's `indexOf` uses CPU SIMD instructions. Files without ` ```mermaid ` are rejected in ~577ns for 100KB.

2. **Trigger-Based Fast Path** - Most diagrams are clean. Quick checks for `<`, `graph `, `$` etc. skip full parsing.

3. **Trie-Based Arrow Matching** - O(arrow_length) instead of O(n × arrow_count) for 25+ arrow types.

4. **Character Lookup Tables** - `Uint8Array[128]` for O(1) character classification instead of regex.

See [PERFORMANCE.md](docs/PERFORMANCE.md) for the full deep-dive.

## CI Integration

```yaml
# GitHub Actions
- name: Lint Mermaid diagrams
  run: npx mermaid-lint check docs/ --json
```

Exit codes:
- `0` - No errors
- `1` - Errors found

## License

MIT
