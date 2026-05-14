export type MediaMetadata = {
  title: string;
  alt: string;
  latitude: number | null;
  longitude: number | null;
};

export type FileEntry = {
  path: string;
  name: string;
  metadata: MediaMetadata;
};

export type Mode = 'shared' | 'per-image';

export function emptyMetadata(): MediaMetadata {
  return { title: '', alt: '', latitude: null, longitude: null };
}

export const app = $state({
  files: [] as FileEntry[],
  selectedIndex: null as number | null,
  mode: 'shared' as Mode,
  shared: emptyMetadata()
});

export function selectedMetadata(): MediaMetadata {
  if (app.mode === 'shared') return app.shared;
  if (app.selectedIndex === null) return emptyMetadata();
  const f = app.files[app.selectedIndex];
  return f ? f.metadata : emptyMetadata();
}

export function addFiles(entries: { path: string; name: string }[]) {
  const existing = new Set(app.files.map((f) => f.path));
  let added = 0;
  for (const e of entries) {
    if (existing.has(e.path)) continue;
    app.files.push({ path: e.path, name: e.name, metadata: emptyMetadata() });
    added += 1;
  }
  if (app.selectedIndex === null && app.files.length > 0) {
    app.selectedIndex = 0;
  }
  return added;
}

export function clearFiles() {
  app.files = [];
  app.selectedIndex = null;
}

export function selectFile(index: number) {
  if (index >= 0 && index < app.files.length) {
    app.selectedIndex = index;
  }
}
