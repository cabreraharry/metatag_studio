### Legacy Python spec — superseded

These four documents describe the **original Python + PySide6 + PyInstaller** design for MetaTag Studio. The project pivoted to **Tauri 2 + Svelte 5 + Rust** before any code was written.

They are kept here because parts of them are still authoritative regardless of stack:

- [SPECIFICATION.md](SPECIFICATION.md)
  - **§2 Scope** — what the app does and doesn't do. Still applies.
  - **§4 Metadata fields & where they are written** — the EXIF/XMP/IPTC tag table is the source of truth for `engine.rs`. Still applies.
  - **§5 Functional requirements** — the UX contract (drag-drop, multi-select, apply-to-all, presets, auto-rename, CSV). Still applies; only the Python-specific module names in §5.7/§5.8 are obsolete.
  - **§6 Non-functional requirements** — UI quality bar, no telemetry, originals safety. Still applies.
  - **§7 Designing for video** — extension-point strategy. Still applies.
  - **§9 Acceptance criteria** — the gate for v1 done. Still applies in full.
- [BUILD_GUIDE.md](BUILD_GUIDE.md) — §1, §7 manual test checklist still apply. §2-§6 (Python venv, PyInstaller, Python-specific CI) are obsolete; see the top-level Tauri build guide.
- [STARTER_NOTES.md](STARTER_NOTES.md) — the Python prototype zip referenced here was never used. The GPS-sign landmine called out in "Known issue already fixed" is preserved in [../../CLAUDE.md](../../CLAUDE.md).
- [CLAUDE_CODE_KICKOFF.md](CLAUDE_CODE_KICKOFF.md) — historical only. The user's actual priorities (polished UI, shareable Windows `.exe`, never modify originals, signed GPS) are restated in the current top-level docs.

For Tauri-stack guidance see the top-level [CLAUDE.md](../../CLAUDE.md) and [README.md](../../README.md).
