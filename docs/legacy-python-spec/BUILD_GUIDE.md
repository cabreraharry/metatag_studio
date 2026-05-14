# Build & Setup Guide

Practical setup, run, and packaging instructions for MetaTag Studio. This
complements `SPECIFICATION.md` (the *what*) with the *how*.

---

## 1. Development environment

**Requirements on the dev machine:**
- Python 3.11 or newer
- Git
- ExifTool (for running from source — the packaged app bundles its own copy)

**ExifTool install:**
- **Windows:** download the Windows package from <https://exiftool.org>. It
  arrives as `exiftool(-k).exe`. Rename it to `exiftool.exe` and either put it
  on your `PATH` or drop it in `metatag/resources/`.
- **macOS:** `brew install exiftool`
- **Linux:** `sudo apt install libimage-exiftool-perl`

Verify: `exiftool -ver` should print a version number.

---

## 2. First-time project setup

```bash
git clone https://github.com/YOUR_USERNAME/metatag-studio.git
cd metatag-studio

python -m venv .venv
# Windows:
.venv\Scripts\activate
# macOS / Linux:
source .venv/bin/activate

pip install -r requirements.txt
```

`requirements.txt` should contain:
```
PySide6>=6.6.0
Pillow>=10.0.0
```

PyInstaller is a build-only dependency — install it separately when packaging,
not in `requirements.txt`.

---

## 3. Running from source

```bash
python -m metatag
```

The GUI window should open. If you see an "ExifTool not found" dialog, fix the
ExifTool install (§1) — the app still opens but can't process files until it's
found.

---

## 4. Packaging the Windows .exe

> Must be done **on Windows**. PyInstaller does not cross-compile — a Windows
> `.exe` can only be built on a Windows machine (or a Windows CI runner).

### 4.1 Bundle ExifTool first
Place the ExifTool executable where the spec file expects it:
```
metatag/resources/exiftool.exe
metatag/resources/exiftool_files/      # the Perl-lib folder from the ExifTool zip
```
If you skip this, the app still builds but end users will need ExifTool on
their own PATH — which defeats the point of a standalone `.exe`.

### 4.2 Build
```bash
pip install pyinstaller
pyinstaller MetaTagStudio.spec
```
Output: `dist/MetaTagStudio/MetaTagStudio.exe` plus its support files.

### 4.3 Test the build
Copy the whole `dist/MetaTagStudio/` folder to a machine (or VM) that has
**neither Python nor ExifTool installed** and confirm:
- the app launches,
- it does not show the "ExifTool not found" dialog,
- it can tag a test image.

This clean-machine test is the only way to be sure the bundle is complete.

---

## 5. The PyInstaller spec file

Key points the `.spec` must get right:
- **Entry point:** `run.py` (PyInstaller wants a script, not `-m metatag`).
- **Data files:** everything in `metatag/resources/` must be added to `datas`
  so the bundled ExifTool ships inside the app.
- **`console=False`:** this is a GUI app; no terminal window should appear.
- **Icon:** point at `metatag/resources/icon.ico` if present, else `None`.
- **Excludes:** trim `tkinter`, `matplotlib`, `numpy` etc. to keep size down.

At runtime, `toolpath.py` must locate the bundled ExifTool via
`sys._MEIPASS` when the app is frozen (PyInstaller unpacks data files to a
temp dir exposed there).

---

## 6. GitHub setup

### 6.1 Create the repo
```bash
git init
git add .
git commit -m "Initial commit: MetaTag Studio"
git branch -M main
git remote add origin https://github.com/YOUR_USERNAME/metatag-studio.git
git push -u origin main
```

### 6.2 What `.gitignore` must exclude
- `__pycache__/`, `*.pyc`
- `.venv/`, `build/`, `dist/`
- `metatag/resources/exiftool.exe` and `metatag/resources/exiftool_files/` —
  **the bundled ExifTool binary should not be committed**; CI downloads it
  fresh at build time. (Committing third-party binaries bloats the repo and
  muddies licensing.)

### 6.3 CI workflow (`.github/workflows/build.yml`)
The workflow should:
1. Trigger on pushes to `main`, on `v*` tags, and via manual dispatch.
2. Run on `windows-latest`.
3. Set up Python 3.12, `pip install -r requirements.txt` + `pyinstaller`.
4. **Download ExifTool** from exiftool.org, unzip it, rename
   `exiftool(-k).exe` → `exiftool.exe`, and copy it (plus `exiftool_files/`)
   into `metatag/resources/`.
5. Run `pyinstaller MetaTagStudio.spec`.
6. Zip `dist/MetaTagStudio/` and upload it as a workflow artifact.
7. On tag pushes only, also attach the zip to a GitHub Release.

Result: every push gives you a downloadable build in the **Actions** tab, and
tagging `v1.0.0` publishes a proper release with the `.exe` attached.

### 6.4 Cutting a release
```bash
git tag v1.0.0
git push origin v1.0.0
```
Watch the Actions tab; the release appears under **Releases** when it finishes.

---

## 7. Manual test checklist

Before considering a build shippable, walk through this by hand:

- [ ] App launches from source (`python -m metatag`).
- [ ] Drag 3–4 JPEGs in — thumbnails and names appear.
- [ ] Drag a *folder* in — its images are added.
- [ ] Select one file, type alt text + title + keywords, pick a GPS preset,
      click **Save to This File**.
- [ ] Select multiple files, enter shared values, click
      **Apply to All Selected**.
- [ ] Set an output folder, click **Process All**, watch the progress bar.
- [ ] Run `exiftool dist-output/somefile.jpg` — confirm description, title,
      keywords, copyright, creator, and **signed** GPS are all present.
- [ ] Confirm the source files are unchanged (compare modified-times / hashes).
- [ ] Tick **Auto-rename**, set prefix `Hallandale`, process again — confirm
      slugged filenames like `hallandale-....jpeg`.
- [ ] Export CSV, edit a value in a spreadsheet, import it back — confirm the
      change loads.
- [ ] Try the address lookup with a real address — confirm lat/lon fill in.
- [ ] Build the `.exe`, run it on a clean machine — confirm it works without
      Python or ExifTool installed.
