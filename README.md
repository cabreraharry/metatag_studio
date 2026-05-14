### MetaTag Studio

A polished desktop app for adding alt text, GPS geotags, and SEO metadata to images. Built for US Prime Realty's listing workflow.

Drag images or a folder into the window, fill in the fields once, click **Process All**, and get correctly tagged copies in your output folder. Originals are never touched.

#### Status

v0.1.0 — early development. Slice 1 (single-file alt-text vertical) lives in [src/routes/+page.svelte](src/routes/+page.svelte) and [src-tauri/src/engine.rs](src-tauri/src/engine.rs).

#### Tech stack

- **Tauri 2** — small native Windows `.exe`
- **Svelte 5 + SvelteKit (SPA mode)** + TypeScript — frontend
- **Rust** — backend (file I/O, ExifTool orchestration, Nominatim geocoding)
- **ExifTool** — bundled as a sidecar binary; the single source of truth for EXIF/XMP/IPTC writes

#### Development setup

Prereqs (Windows):
- Node.js 22+
- Rust 1.78+ (`rustup`)
- Microsoft Visual Studio Build Tools (C++ workload) — Tauri needs MSVC
- WebView2 (preinstalled on Windows 11; via Edge on Windows 10)

```powershell
git clone https://github.com/cabreraharry/metatag_studio.git
cd metatag_studio
npm install

# Download ExifTool Windows distribution into src-tauri/binaries/ and src-tauri/resources/
# (see scripts/fetch-exiftool.ps1 or download manually from https://exiftool.org)

npm run tauri dev
```

#### Build a shareable installer

```powershell
npm run tauri build
# Output: src-tauri/target/release/bundle/nsis/MetaTag Studio_*.exe
```

The NSIS installer installs per-user (`%LOCALAPPDATA%\Programs\MetaTag Studio\`), no admin prompt. Expected size ~30–35 MB.

#### Documentation

- [CLAUDE.md](CLAUDE.md) — guidance for AI assistants working in this repo
- [docs/legacy-python-spec/](docs/legacy-python-spec/) — the original Python/PySide6 spec; the metadata-field tables (§4) and acceptance criteria (§9) are still authoritative

#### License

Proprietary. See [LICENSE](LICENSE). Bundled ExifTool is distributed under its own license (Perl Artistic / GPL).
