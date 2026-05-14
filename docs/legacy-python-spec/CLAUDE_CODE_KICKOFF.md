# Kickoff Prompt for Claude Code

Copy everything in the box below and paste it as your first message to Claude
Code, after placing `SPECIFICATION.md`, `BUILD_GUIDE.md`, and `STARTER_NOTES.md`
in your project folder. (Optionally also drop in the `metatag-studio.zip`
prototype from the earlier session.)

---

```
I want to build a desktop app called MetaTag Studio. The full spec is in
SPECIFICATION.md, build instructions in BUILD_GUIDE.md, and notes on an
existing prototype in STARTER_NOTES.md — please read all three before starting.

Short version: it's a PySide6 desktop app that adds alt text, GPS geotags, and
SEO metadata (title, keywords, copyright, creator) to image files, using
ExifTool under the hood. It needs to be packaged as a standalone Windows .exe
and hosted on GitHub with CI that builds the .exe automatically.

I have a prototype from an earlier session (in metatag-studio.zip if I've added
it to the folder) — the backend/engine is tested and working, but the GUI has
never actually been displayed on a screen. Getting the UI/UX genuinely good is
my top priority.

My environment: Windows, Python [FILL IN VERSION], ExifTool [installed? / not yet].

Please start by:
1. Reading the three spec docs.
2. Confirming the open questions in SPECIFICATION.md section 10 with me.
3. Setting up the project (use the prototype as a base if available, else build
   fresh from the spec).
4. Running the GUI so we can both see it, then iterating on the layout together.

Don't build the whole thing in one shot — let's go module by module, run things
as we go, and get the UI right before packaging. Ask me questions whenever the
spec is ambiguous.
```

---

## Notes for you (the user) before you paste

- **Fill in the bracketed bits** — your Python version, and whether ExifTool is
  installed yet. Run `python --version` and `exiftool -ver` to check.
- **Decide on the open questions** ahead of time if you can — they're in
  `SPECIFICATION.md` §10 (license, presets, one-file vs one-folder build, app
  icon). Claude Code will ask, but having answers ready speeds things up.
- **Add the prototype zip to the folder** if you have it — it saves Claude Code
  from rebuilding tested code. If you don't, no problem; the spec is complete
  enough to build from scratch.
- **Expect iteration on the UI.** The backend is solid, but the visual side
  needs real eyes on a real screen. Budget time to look at it together and say
  "make this bigger / move that / this is confusing."
- **The .exe build may need debugging.** PyInstaller specs often need a tweak
  or two on first real run. That's normal — Claude Code can iterate on it
  locally, which is exactly why running it on your PC is the right move.
