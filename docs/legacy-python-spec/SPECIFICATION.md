# MetaTag Studio — Project Specification

> A desktop application for adding alt text, GPS geotags, and SEO metadata to
> images (and later, videos). This document is the build specification for
> Claude Code.

---

## 1. Purpose & context

The user runs a real estate brokerage (US Prime Realty, Florida) and regularly
publishes large numbers of property and team photos. They need those photos
properly tagged for **search engine optimization** and **accessibility** before
they go on the website or into listing platforms.

Doing this by hand, or with one-off scripts, is slow and error-prone. The goal
is a polished, reusable desktop tool that a non-technical person can open, drag
photos into, fill in a few fields, and get back correctly tagged files.

**Primary user:** non-developer. The UI must be obvious without a manual.

---

## 2. Scope

### In scope (v1)
- Images only: `.jpg`, `.jpeg`, `.png`, `.tif`, `.tiff`, `.heic`, `.heif`, `.webp`
- Writing: alt text/description, title, keywords, copyright, creator, GPS
- Reading existing metadata back into the editor
- Per-image editing and bulk ("apply to many") editing
- CSV import/export for spreadsheet-based bulk workflows
- Auto-renaming files from their description (SEO-friendly slugs)
- Saved GPS location presets + address-to-coordinates lookup
- Packaged as a standalone Windows `.exe`, plus runnable from source
- Hosted on GitHub with CI that builds the `.exe` automatically

### Out of scope (v1) — but design for it
- Video files (`.mp4`, `.mov`, `.m4v`). Leave clean extension points so video
  can be added later without restructuring. See §7.
- Cloud sync, accounts, or any server component
- Editing the actual image pixels (cropping, resizing, etc.)

---

## 3. Tech stack

| Concern        | Choice                  | Notes |
|----------------|-------------------------|-------|
| Language       | Python 3.11+            | User's preferred language |
| GUI framework  | PySide6 (Qt for Python) | Mature, good-looking, easy to package |
| Metadata I/O   | ExifTool (bundled)      | Industry standard; do not reimplement |
| Packaging      | PyInstaller             | Produces the Windows `.exe` |
| CI             | GitHub Actions          | Builds `.exe` on Windows runners |
| Image thumbs   | Pillow                  | For list thumbnails |

**Do not** try to parse or write EXIF/XMP/IPTC by hand. Shell out to ExifTool.
It is the single source of truth for metadata operations.

---

## 4. Metadata fields & where they are written

ExifTool writes the same logical value into multiple metadata standards so that
any downstream tool can read it. For each field below, write to **all** listed
tags.

| Field      | Tags to write |
|------------|---------------|
| Alt text   | `EXIF:ImageDescription`, `XMP-dc:Description`, `IPTC:Caption-Abstract` |
| Title      | `XMP-dc:Title`, `IPTC:ObjectName` |
| Keywords   | `XMP-dc:Subject`, `IPTC:Keywords` (multi-value; clear before re-adding) |
| Copyright  | `EXIF:Copyright`, `XMP-dc:Rights`, `IPTC:CopyrightNotice` |
| Creator    | `EXIF:Artist`, `XMP-dc:Creator`, `IPTC:By-line` |
| GPS        | `EXIF:GPSLatitude` + `EXIF:GPSLatitudeRef`, `EXIF:GPSLongitude` + `EXIF:GPSLongitudeRef`, mirrored to `XMP:GPSLatitude` / `XMP:GPSLongitude` |

### GPS specifics
- Input is **signed decimal degrees** (e.g. `25.9812`, `-80.1484`).
- EXIF stores magnitude + a reference letter: latitude `N`/`S`, longitude `E`/`W`.
  Positive latitude → `N`, negative → `S`; positive longitude → `E`,
  negative → `W`.
- **When reading GPS back**, request ExifTool's `Composite:GPSLatitude` /
  `Composite:GPSLongitude` tags — these return already-signed decimals. Reading
  the raw `EXIF:GPSLongitude` alone loses the sign. (This was a real bug in an
  earlier prototype — do not repeat it.)
- Always use ExifTool's `-n` flag for numeric (not human-readable) values.

### Writing safety
- **Never modify the original file.** Copy the source to the output folder
  first (`shutil.copy2` to preserve timestamps), then run ExifTool on the copy
  with `-overwrite_original` (which now only affects the copy).
- Use `-codedcharacterset=utf8` so accented characters survive.

---

## 5. Functional requirements

### 5.1 Adding files
- Drag-and-drop files onto the file list.
- Dragging a **folder** adds all supported images one level deep.
- An **Add...** button opens a file picker as an alternative.
- Adding a file already in the list is a no-op (dedupe by resolved path).
- On add, immediately read existing metadata and show it — users should see
  what's already there, not a blank slate that risks wiping data.

### 5.2 The file list (left panel)
- Shows a thumbnail + filename per file.
- Multi-select enabled (shift/ctrl click).
- **Remove** (selected) and **Clear** (all) buttons.

### 5.3 The metadata editor (right panel)
- Fields: Title, Alt text (multi-line), Keywords (comma-separated input),
  Creator, Copyright.
- Location group: a preset dropdown, manual lat/lon inputs, and an address
  field with a **Look up** button.
- **Single selection:** editor shows that file's metadata; edits target it.
- **Multi selection:** editor starts blank; an **Apply to All Selected** button
  writes the entered values to every selected file.
- **Save to This File** button persists edits for the single active file.
- Editing is in-memory until **Process All** is run — nothing touches disk
  before that. (Exception: the editor's saved state is what gets processed.)

### 5.4 GPS / location
- **Presets:** a dropdown of saved named locations. Ship three example presets:
  `Hallandale, FL` (25.9812, -80.1484), `Naples, FL` (26.1420, -81.7948),
  `Orlando, FL` (28.5384, -81.3789). Selecting one fills lat/lon.
- Presets persist between sessions as JSON in the OS user-config directory
  (`%APPDATA%/MetaTagStudio/` on Windows). A first run seeds the defaults.
- **Bonus (nice-to-have):** a small dialog to add/edit/delete presets.
- **Address lookup:** type an address, click **Look up**, the app geocodes it
  via OpenStreetMap Nominatim and fills lat/lon. This needs internet; fail
  gracefully with a clear message and let the user type coordinates manually.
  Nominatim requires a descriptive `User-Agent` header — set one.

### 5.5 Auto-rename
- An **Auto-rename from description** checkbox.
- When on, output files are renamed to a slug of the alt text (fallback: title,
  then original stem): lowercase, ASCII-only, spaces→hyphens, punctuation
  stripped, accents flattened (`café`→`cafe`), capped at ~8 words.
- An optional **prefix** field (e.g. `Hallandale`) is slugified and prepended:
  `hallandale-office-interior.jpeg`.
- On filename collisions within one batch, append `-2`, `-3`, etc.

### 5.6 CSV import/export
- **Export CSV:** writes a row per loaded file with columns
  `filename,title,alt_text,keywords,copyright,creator,latitude,longitude`.
  Keywords are joined with `;` inside the single cell.
- **Import CSV:** reads that format back; matches rows to loaded files by
  `filename`. Unknown extra columns are ignored (users may keep notes columns).
  Missing the `filename` column is an error shown to the user.

### 5.7 Processing
- A **Choose Output...** button sets the destination folder.
- **Process All** validates (engine ready? files present? output set?) then
  runs every loaded file through the engine **on a background thread** so the
  UI stays responsive. Show a progress bar.
- On completion: a summary (`N succeeded, M failed`). If any failed, show the
  per-file error messages.

### 5.8 ExifTool discovery
- The app looks for ExifTool in this order:
  1. A bundled copy under `metatag/resources/` (this is what ships in the `.exe`).
  2. ExifTool on the system `PATH`.
- If neither is found, show a friendly, non-fatal dialog explaining how to
  install it. The app should still open.
- Handle PyInstaller's frozen-path case (`sys._MEIPASS`) when locating the
  bundled copy.

---

## 6. Non-functional requirements

- **UI quality matters.** This is the user's explicit priority. Clean spacing,
  sensible default window size (~1040×680), logical tab order, real
  placeholder text, keyboard shortcuts for common actions. It should feel like
  a finished product, not a script with buttons.
- **Responsiveness:** no operation blocks the UI thread. ExifTool calls and
  geocoding both run off-thread or are fast enough not to matter.
- **Safety:** originals are never written to. This is non-negotiable.
- **Cross-platform-friendly code**, even though only Windows is packaged.
  Running from source should work on macOS/Linux too.
- **No telemetry, no network calls** except the explicit address-lookup feature.

---

## 7. Designing for video (future-proofing)

Do not implement video, but structure the code so it slots in cleanly:
- Keep a `SUPPORTED_VIDEO_EXTS` set alongside `SUPPORTED_IMAGE_EXTS`.
- The metadata engine should branch on file type. Provide a `build_video_args()`
  method (even if unused) so the writing path is already shaped for it.
- Video metadata differs: descriptions go to QuickTime/Keys/ItemList atoms;
  GPS uses an ISO 6709 string (`+25.9812-080.1484/`) in
  `UserData:GPSCoordinates`, not EXIF tags.
- The GUI's file-handling and worker code should not hard-assume "image".

---

## 8. Project structure

```
metatag-studio/
├── metatag/
│   ├── __init__.py        # version, app name constants
│   ├── __main__.py        # `python -m metatag` entry point
│   ├── engine.py          # ExifTool wrapper: MediaMetadata + MetadataEngine
│   ├── mainwindow.py      # the PySide6 main window / all UI
│   ├── worker.py          # QThread that processes files in the background
│   ├── naming.py          # slugify + filename building
│   ├── locations.py       # LocationPreset store + geocode()
│   ├── batch.py           # CSV import/export
│   ├── toolpath.py        # find_exiftool() — bundled or PATH
│   └── resources/         # icon.ico + bundled exiftool (gitignored)
├── run.py                 # PyInstaller entry (calls metatag.__main__.main)
├── MetaTagStudio.spec     # PyInstaller build config
├── requirements.txt
├── .gitignore
├── LICENSE                # MIT (note bundled ExifTool's own license)
├── README.md
└── .github/workflows/
    └── build.yml          # CI: build .exe on push/tag
```

A previous prototype of all these files exists and can be used as a starting
reference — see `STARTER_NOTES.md`. It compiles and the engine is tested, but
the GUI has not been visually verified. Treat it as a draft to improve, not
gospel.

---

## 9. Acceptance criteria

The build is done when:

1. `python -m metatag` launches the GUI on the user's PC.
2. Dragging in a few JPEGs, filling alt text + selecting a GPS preset, choosing
   an output folder, and clicking **Process All** produces tagged copies whose
   metadata is confirmed correct via `exiftool <file>` on the command line.
3. GPS reads back with the **correct sign** (western longitudes negative).
4. **Apply to All Selected** correctly tags a multi-file selection in one pass.
5. **Auto-rename** produces clean slugs with the optional prefix, and handles
   collisions.
6. CSV export → edit → import round-trips without data loss.
7. Originals in the source folder are byte-for-byte unchanged after processing.
8. `pyinstaller MetaTagStudio.spec` produces a working `dist/MetaTagStudio/`
   that runs on a clean Windows machine **without** Python or ExifTool
   installed.
9. The GitHub Actions workflow runs green and uploads the `.exe` artifact.
10. README is accurate enough that the user could set this up from scratch.

---

## 10. Open questions to confirm with the user

Claude Code should ask these early if not already answered:
- Is the MIT license correct, or do they want the repo private with all rights
  reserved?
- Should the three FL cities be the shipped presets, or different ones?
- Is a single-folder PyInstaller build fine, or do they specifically want a
  one-file `.exe` (slower to start, but tidier)?
- Do they want an app icon designed, or is a placeholder fine for v1?
