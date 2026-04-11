# Handoff

## Repo

- Path: `/data/codex`
- Branch: `main`
- `origin`: `https://github.com/hitori-chan/codex.git`
- `upstream`: `https://github.com/openai/codex.git`

## Current Sync State

- Latest upstream synced base:
  - `66e13efd9` `TUI: enforce core boundary (#17399)`
- Fork release version prepared:
  - `0.120.1`
- Fork release tag to create:
  - `rust-v0.120.1`

## Fork-Specific Changes Kept

- `/autonomous` feature still present
- default stop token: `AUTONOMOUS_DONE`
- stop detection uses `contains`, not exact match
- fork-safe `rust-release.yml` still used
- non-release workflows remain manual-only (`workflow_dispatch`)

## CI Policy

- This fork optimized for releases, not full upstream CI
- Kept active:
  - `.github/workflows/rust-release.yml`
- Manual only:
  - `.github/workflows/ci.yml`
  - `.github/workflows/bazel.yml`
  - `.github/workflows/rust-ci.yml`
  - `.github/workflows/rust-ci-full.yml`
  - `.github/workflows/cargo-deny.yml`
  - `.github/workflows/codespell.yml`

## Last Published Release

- version: `0.118.8`
- tag: `rust-v0.118.8`
- URL:
  - `https://github.com/hitori-chan/codex/releases/tag/rust-v0.118.8`

## Local Validation For Current Sync

- `just fix -p codex-exec`
- `just fix -p codex-tui`
- `just fmt`
- `cargo test -p codex-exec`
- `cargo test -p codex-tui autonomous_turn_complete_stops_when_stop_message_matches`

## Notes For Next Agent

- Upstream `main` currently carries workspace version `0.0.0`; fork release must bump version before tagging
- If release workflow breaks, first check `.github/workflows/rust-release.yml`; upstream reusable release pieces still not suitable for this fork
- If re-enabling Bazel/full CI later, fix fork fallback config first

## Useful Commands

```bash
git -C /data/codex status --short --branch
git -C /data/codex log --oneline --decorate -n 12
gh run list --repo hitori-chan/codex --limit 10
gh release view rust-v0.118.8 --repo hitori-chan/codex
```
