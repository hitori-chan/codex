# Handoff

## Repo

- Path: `/home/bocchi/project/codex`
- Branch: `main`
- Remote: `origin = https://github.com/hitori-chan/codex.git`
- Upstream is not configured as a named remote; fetches were done directly from:
  - `https://github.com/openai/codex.git`

## Current State

- `main` / `origin/main` / `rust-v0.117.0` currently point to:
  - `83d9403e52e10a3253364a1b1c2da2ced7aacda3`
  - `Merge branch 'main' of https://github.com/openai/codex`
- Latest upstream commit synced into this branch:
  - `f044ca64df3f13faa679341aeb4469706c458f4d`
  - `Use codex-utils-template for search tool descriptions (#15996)`
- Release version intentionally remains:
  - `0.117.0`

## Recent Sync History

Fork-specific commits retained in history:

1. `232bf1aa3` `Add autonomous prompt mode and fork release flow`
2. `48625a8d2` `Fix app-server autonomous mode parity`
3. `abd86efe1` `Merge branch 'main' of https://github.com/openai/codex`
4. `5cdb3930c` `Restore generated app-server schema fixtures`
5. `a431aa88e` `Merge branch 'main' of https://github.com/openai/codex`
6. `83d9403e5` `Merge branch 'main' of https://github.com/openai/codex`

Latest upstream commits merged during the last two sync cycles:

- `270b7655c` `Use codex-utils-template for login error page (#16000)`
- `7d5d9f041` `Use codex-utils-template for review prompts (#16001)`
- `2c85ca684` `Use codex-utils-template for sandbox mode prompts (#15998)`
- `37b057f00` `Use codex-utils-template for collaboration mode presets (#15995)`
- `f044ca64d` `Use codex-utils-template for search tool descriptions (#15996)`

## GitHub Release / CI

Current published release:

- release: `0.117.0`
- tag: `rust-v0.117.0`
- URL: `https://github.com/hitori-chan/codex/releases/tag/rust-v0.117.0`

Latest successful GitHub runs for the current head:

- `rust-release` success:
  - run `23651664411`
- `rust-ci` success:
  - run `23651637812`
- `cargo-deny` success:
  - run `23651637880`
- `Codespell` success:
  - run `23651637818`
- lightweight `ci` success:
  - run `23651637878`

## Local Validation In This Session

Passed:

- `just fmt`
- `cargo test -p codex-app-server-protocol`
- `cargo test -p codex-core spec_tests`
  - note: this filtered run rebuilt the changed core path but matched no runnable tests
- `./tools/argument-comment-lint/run.sh`
- `just bazel-lock-check`

Important note about one earlier local core run:

- `cargo test -p codex-core` was tried earlier and failed in unrelated integration tests because the test harness could not locate the `test_stdio_server` binary under local `target/debug`
- that failure did not reproduce in GitHub CI; `rust-ci` run `23651637812` succeeded on the final pushed head

## Backup Branches

Useful local safety branches currently present:

- `backup_before_sync_20260327`
- `backup_before_sync_20260327_release_retry`
- `backup_before_sync_20260327_release_retry_2`
- `backup_before_sync_20260327_release_retry_3`

## Worktree Status At Handoff

- tracked files clean after committing this handoff

## Notes For The Next Agent

- The branch is no longer a simple squashed fork; it now contains fork commits plus upstream merge commits.
- The current release tag `rust-v0.117.0` is already published successfully on `83d9403e5`.
- If releasing again without changing the version, the existing GitHub release/tag must be deleted and recreated.
- If bumping the version instead, update `codex-rs/Cargo.toml`, refresh any derived lock/schema artifacts as needed, push `main`, and tag the matching `rust-v<version>`.

## Useful Commands

Check current status:

```bash
git -C /home/bocchi/project/codex status --short --branch
```

Check recent upstream-sync history:

```bash
git -C /home/bocchi/project/codex log --oneline --decorate -n 12
```

Check recent GitHub runs:

```bash
gh run list --repo hitori-chan/codex --limit 10
```

Check the current release:

```bash
gh release view rust-v0.117.0 --repo hitori-chan/codex
```
