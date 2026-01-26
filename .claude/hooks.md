---
name: dev-workflow-hooks
description: Development workflow hooks for git_activity_dashboard project
user-invocable: false
hooks:
  PreToolUse:
    - command: |
        # Run tests before committing
        if [[ "$TOOL_NAME" == "Bash" && "$COMMAND" == git commit* ]]; then
          echo "Running tests before commit..."
          cd git_activity_dashboard && cargo test --quiet 2>&1 | head -20
        fi
      timeout: 30000
      once: true
    - command: |
        # Run clippy before git push
        if [[ "$TOOL_NAME" == "Bash" && "$COMMAND" == git push* ]]; then
          echo "Running clippy before push..."
          cd git_activity_dashboard && cargo clippy --quiet 2>&1 | head -20
        fi
      timeout: 30000
      once: true
---

# Development Workflow Hooks

This file defines hooks for the git_activity_dashboard development workflow.

## PreToolUse Hooks

### Pre-commit Test Hook
Runs `cargo test` before any git commit to ensure tests pass.

### Pre-push Clippy Hook
Runs `cargo clippy` before any git push to catch linting issues.

## Usage

Hooks in this file are automatically loaded. To disable:
- Remove `hooks:` section from frontmatter
- Or set `disableAllHooks: true` in settings.json
