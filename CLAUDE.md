### CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this app is

MetaTag Studio is a Windows desktop app that writes image metadata (alt text, title, keywords, copyright, creator, GPS) via **ExifTool**, packaged as a shareable NSIS installer. Primary user is non-technical (US Prime Realty brokerage staff) — UI quality is the explicitly stated top priority.

## Tech stack

- **Tauri 2** — Rust backend, NSIS installer (`installMode: "currentUser"`)
- **Svelte 5 + SvelteKit** (SPA mode via `adapter-static`) + TypeScript + Vite
- **ExifTool** shelled out as a sidecar binary — never reimplement EXIF/XMP/IPTC by hand

The previous spec under [docs/legacy-python-spec/](docs/legacy-python-spec/) prescribed Python + PySide6. That stack was rejected in favor of Tauri for bundle size and distribution. The metadata-field tables (§4) and acceptance criteria (§9) in that old spec are still authoritative; the rest is historical.

## Commands

```powershell
# Run from source
npm install
npm run tauri dev

# Type-check frontend
npm run check

# Type-check Rust
cd src-tauri ; cargo check

# Package shareable .exe installer
npm run tauri build
# Output: src-tauri/target/release/bundle/nsis/MetaTag Studio_*.exe
```

No automated test suite is set up yet. Verification gate is the manual checklist in [docs/legacy-python-spec/BUILD_GUIDE.md §7](docs/legacy-python-spec/BUILD_GUIDE.md) and the acceptance criteria in [docs/legacy-python-spec/SPECIFICATION.md §9](docs/legacy-python-spec/SPECIFICATION.md). The acceptance gate is `exiftool <file>` on the command line confirming tags round-trip — including **signed** GPS.

## Architecture

Rust modules under [src-tauri/src/](src-tauri/src/):

- [engine.rs](src-tauri/src/engine.rs) — **single source of truth for metadata I/O**. Shells out to ExifTool via `app.shell().sidecar("exiftool")` (Tauri 2 API). Falls back to PATH-installed `exiftool` if the sidecar isn't bundled.
- [commands.rs](src-tauri/src/commands.rs) — `#[tauri::command]` IPC surface; thin wrappers over engine/locations/batch so core logic stays testable without the Tauri runtime.
- [error.rs](src-tauri/src/error.rs) — `AppError` enum that serializes to a string for IPC.
- [naming.rs](src-tauri/src/naming.rs) — slugify + collision suffixes for auto-rename.
- *Coming in Slice 3:* `locations.rs` — preset JSON store at `BaseDirectory::AppConfig` + Nominatim geocoding.
- *Coming in Slice 4:* `batch.rs` — CSV import/export.

Frontend under [src/](src/):
- [src/routes/+page.svelte](src/routes/+page.svelte) — main UI (current Slice 1 single-route layout).
- [src/lib/](src/lib/) — components as they get extracted (FileList, MetadataEditor, LocationPicker, ProgressDialog).

ExifTool ships as a **sidecar binary** at [src-tauri/binaries/exiftool-x86_64-pc-windows-msvc.exe](src-tauri/binaries/) (gitignored — CI/dev downloads it from exiftool.org), with the Perl runtime in [src-tauri/resources/exiftool_files/](src-tauri/resources/) (also gitignored). Both must end up as siblings in the install dir at runtime; Tauri's `externalBin` + `resources` config in [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) handles that.

## Tauri 2 specifics (v1 → v2 transition landmines)

These are the v2 corrections that bite when you bring v1 muscle memory:

1. **Sidecar API:** `app.shell().sidecar("exiftool")` via the `ShellExt` trait — NOT v1's `Command::new_sidecar()`.
2. **Platform-suffix file naming:** the on-disk filename must be `exiftool-x86_64-pc-windows-msvc.exe` even though the config and runtime call refer to just `"exiftool"`.
3. **`exiftool_files/` Perl-lib folder is a `bundle.resources` entry**, not part of `externalBin`. They sit as siblings in the install dir at runtime.
4. **Capabilities file required:** [src-tauri/capabilities/default.json](src-tauri/capabilities/default.json) must whitelist `shell:allow-spawn` (with `name: "binaries/exiftool", sidecar: true`) and `dialog:default`. Missing capabilities fail at runtime, not at compile.
5. **Drag-drop:** `getCurrentWebviewWindow().onDragDropEvent()` — v1's raw `tauri://file-drop` event is gone. Folder drops give the folder path; enumerate one level deep in Rust per spec §5.1.
6. **Config dir:** `app.path().resolve("locations.json", BaseDirectory::AppConfig)` resolves to `%APPDATA%\<bundleIdentifier>\`. Don't change `bundleIdentifier` (`com.usprimerealty.metatagstudio`) after first run — it orphans user data.
7. **No `spawn_blocking`:** `tauri-plugin-shell`'s async APIs are already non-blocking; wrapping them in `spawn_blocking` actively hurts.
8. **No portable single-exe:** NSIS installer with `installMode: "currentUser"` is the closest equivalent. Expected installer size ~30–35 MB (ExifTool's Perl lib is the dominant ~25 MB).
9. **Canonicalize paths with `dunce::canonicalize`** on Windows — `std::fs::canonicalize` returns ugly UNC-prefixed paths.

## Critical landmines (carried forward from spec — do not regress)

1. **GPS sign bug.** Reading raw `EXIF:GPSLongitude` drops the sign — western longitudes come back positive. Always read `Composite:GPSLatitude` / `Composite:GPSLongitude` with `-n`. Acceptance criterion §9.3 exists specifically to catch regressions.
2. **Never modify originals.** Flow is `fs::copy(src → output)` + preserve mtime via `filetime`, then `exiftool -overwrite_original` against the *copy*. SHA-256 verify in tests. Acceptance criterion §9.7 hashes originals.
3. **Write each field to all three standards** (EXIF + XMP + IPTC) per the table in [docs/legacy-python-spec/SPECIFICATION.md §4](docs/legacy-python-spec/SPECIFICATION.md). Keywords are multi-value: clear (`Tag=`) before re-adding (`Tag+=`) in a single invocation to avoid duplicates.
4. **UTF-8 + numeric flags.** Every ExifTool call gets `-codedcharacterset=utf8` and `-n`.
5. **Nominatim User-Agent.** Geocoding via OpenStreetMap requires a descriptive `User-Agent` (e.g. `MetaTagStudio/1.0 (contact@…)`) — generic UAs get 403'd or rate-limited.
6. **No network calls** except the explicit address-lookup feature. No telemetry.
7. **Design for video, don't build it.** Keep `SUPPORTED_VIDEO_EXTS` parallel to `SUPPORTED_IMAGE_EXTS` in [engine.rs](src-tauri/src/engine.rs). Video uses QuickTime atoms and ISO 6709 GPS strings, not EXIF — see [docs/legacy-python-spec/SPECIFICATION.md §7](docs/legacy-python-spec/SPECIFICATION.md).

## Build slice plan (where we are)

Vertical-slice plan from [C:\Users\harrycabrera\.claude\plans\alright-i-want-you-pure-cosmos.md](C:/Users/harrycabrera/.claude/plans/alright-i-want-you-pure-cosmos.md):

- **Slice 0** — scaffold + window opens — ✅ in progress, gate is `npm run tauri dev`.
- **Slice 1** — single-file vertical (drop one JPEG → type alt → process → tagged copy) — ✅ engine.rs + commands.rs + +page.svelte written, awaiting user-side `tauri dev` run.
- **Slice 2** — file list, all fields, Apply-to-All, progress events.
- **Slice 3** — GPS write/read with sign-correctness, location presets, Nominatim.
- **Slice 4** — auto-rename slugs, CSV import/export.
- **Slice 5** — UI polish, CI workflow, packaged NSIS `.exe` tested on clean Windows machine.

## Working style the user expects

- **Run the GUI early and often.** Screenshot-driven iteration on layout. Don't package the `.exe` until the UI looks right on screen.
- **Module by module.** No big-bang builds.
- **Default window size 1040×680**, clean spacing, real placeholder text, logical tab order, keyboard shortcuts (Ctrl+O, Ctrl+S, Ctrl+Enter). Bar: "feels like a finished product, not a script with buttons."
