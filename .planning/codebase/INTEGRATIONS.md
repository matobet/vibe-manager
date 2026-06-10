# External Integrations

**Analysis Date:** 2026-06-10

## APIs & External Services

**None.** Vibe Manager is a fully offline, local-first TUI application. It makes no HTTP requests and connects to no external APIs or cloud services.

## Data Storage

**Databases:**
- None — no database engine used

**File Storage:**
- Local filesystem only
- Workspace root: user-supplied directory path passed as CLI argument (defaults to `.`)
- Workspace marker: `.vibe-manager` YAML config file at workspace root
- Report data: one subdirectory per report, identified by a name-derived slug
  - Profile: `<slug>/_profile.md` (YAML frontmatter + markdown body)
  - Journal entries: `<slug>/journal/YYYY-MM-DDTHHMMSS.md` (new format) or `<slug>/YYYY-MM-DD.md` (legacy format)
  - Manager 2nd-level reports: `<slug>/team/<sub-slug>/` (recursive same structure)
- All reads/writes use `std::fs` directly via repository types in `src/storage/repo/`

**Caching:**
- None — all data loaded fresh from disk on app startup via `App::new()` in `src/app/state.rs`

## Authentication & Identity

**Auth Provider:**
- None — no authentication system; workspace access controlled entirely by filesystem permissions

## Monitoring & Observability

**Error Tracking:**
- None

**Logs:**
- None — errors surface as terminal status messages via `App::set_status()` in `src/app/state.rs`; no persistent log files written

## CI/CD & Deployment

**Hosting:**
- Distributed as standalone native binaries via GitHub Releases

**CI Pipeline:**
- GitHub Actions (`.github/workflows/ci.yml`)
  - `test` job: `cargo test --all-features` on `ubuntu-latest`
  - `clippy` job: `cargo clippy --all-features -- -D warnings` on `ubuntu-latest`
  - `fmt` job: `cargo fmt --all -- --check` on `ubuntu-latest`
  - `coverage` job: `cargo llvm-cov --all-features --html` via `taiki-e/install-action@cargo-llvm-cov`; coverage HTML uploaded as artifact (14-day retention)
  - Triggers: push to `main`, pull requests to `main`

**Release Pipeline:**
- GitHub Actions (`.github/workflows/release.yml`)
  - Triggered on `v*` tag push
  - Builds for 5 targets: `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc`
  - aarch64 Linux uses `cross` for cross-compilation
  - Creates GitHub Release with SHA-256 checksums via `softprops/action-gh-release@v1`
  - Uses `secrets.GITHUB_TOKEN` for release creation (no additional secrets required)

## Environment Configuration

**Required env vars:**
- None required for normal operation

**Optional env vars:**
- `$EDITOR` - Preferred text editor; checked first by `src/editor.rs`
- `$VISUAL` - Fallback editor if `$EDITOR` is unset or empty
- If neither is set, editor resolution falls back to: `nano` → `vim` → `vi` (checked via `which`)

**Secrets location:**
- No application secrets — `GITHUB_TOKEN` is the only credential used and is injected automatically by GitHub Actions

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- None

## External Process Integration

**External Editor:**
- The application suspends the TUI (disables raw mode, leaves alternate screen) and delegates note editing to an external process
- Resolved via `src/editor.rs` `get_editor()` function at time of edit
- Modification detected via filesystem mtime comparison before/after editor process exit
- Common usage: `$EDITOR=nvim`, `$EDITOR=code --wait`, `$EDITOR=vim`

---

*Integration audit: 2026-06-10*
