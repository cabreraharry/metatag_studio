# Starter Notes — Existing Prototype

A working draft of this project was already built in a previous session. You
(Claude Code) can ask the user for the `metatag-studio.zip` from that session
to use as a starting point, **or** build fresh from `SPECIFICATION.md`. Either
is fine — but read this first so you know the state of that draft.

---

## What the prototype contains

A complete file tree matching §8 of the spec: all nine `metatag/` modules,
`run.py`, `MetaTagStudio.spec`, `requirements.txt`, `.gitignore`, `LICENSE`,
`README.md`, and the CI workflow.

## What is verified

- **Every module compiles** (`py_compile` clean).
- **The metadata engine is functionally tested** — write → read-back
  round-trips correctly for alt text, title, keywords, copyright, creator, and
  GPS. CSV export/import round-trips. Empty-metadata writes don't error.
- **The GUI instantiates** without error in Qt's offscreen mode (catches
  missing names and bad signal wiring).

## What is NOT verified

- **The GUI has never been displayed.** Layout, spacing, sizing, tab order,
  and actual interaction flow are unverified. This is the single biggest area
  needing attention — and it happens to be the user's stated top priority
  ("a really good UI/UX"). Plan to run it, look at it, and refine.
- The PyInstaller build has **never been run** — the prototype was built in a
  headless Linux environment. The `.spec` is written from documentation, not
  from a successful build. Expect to debug it.
- The CI workflow is likewise untested against a real GitHub runner.

## Known issue already fixed (don't reintroduce)

The engine originally read GPS back via raw `EXIF:GPSLongitude`, which **drops
the sign** — a western longitude came back positive. The fix: read
`Composite:GPSLatitude` / `Composite:GPSLongitude` with `-n`, which return
already-signed decimals. If you rebuild the engine from scratch, get this
right from the start. There is a regression-worthy test for it in the spec's
acceptance criteria (§9.3).

## Suggested approach for Claude Code

1. Get the prototype zip from the user (or rebuild from spec).
2. Set up the environment per `BUILD_GUIDE.md` §2.
3. **Run the GUI first thing** (`python -m metatag`) and actually look at it.
   Screenshot-driven iteration on the layout is the highest-value work here.
4. Walk the manual test checklist in `BUILD_GUIDE.md` §7.
5. Get the PyInstaller build working on the user's Windows machine.
6. Initialize the GitHub repo, push, confirm CI goes green.
7. Refine UI/UX based on what you see — this is expected, not optional.

## Things the prototype does well — keep these

- Clean separation of concerns (engine / GUI / worker / helpers).
- Originals are never modified — files are copied, then the copy is tagged.
- Writes each field to EXIF + XMP + IPTC for maximum compatibility.
- ExifTool discovery handles both bundled and PATH cases, including the
  PyInstaller frozen-path case.
- Video extension points are scaffolded without being implemented.

Treat the prototype as a solid skeleton with one untested limb (the UI). Your
job is to bring it to life on a real screen and into a real `.exe`.
