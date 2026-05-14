<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { open } from '@tauri-apps/plugin-dialog';
  import { onDestroy, onMount } from 'svelte';

  import FileList from '$lib/FileList.svelte';
  import ModeToggle from '$lib/ModeToggle.svelte';
  import MetadataEditor from '$lib/MetadataEditor.svelte';
  import { app, addFiles } from '$lib/state.svelte';

  let dragOver = $state(false);
  let busy = $state(false);
  let summary = $state<{ ok: number; failed: { name: string; error: string }[] } | null>(null);

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    const win = getCurrentWebviewWindow();
    unlisten = await win.onDragDropEvent((event) => {
      if (event.payload.type === 'over') {
        dragOver = true;
      } else if (event.payload.type === 'leave') {
        dragOver = false;
      } else if (event.payload.type === 'drop') {
        dragOver = false;
        void addPaths(event.payload.paths);
      }
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });

  async function addPaths(paths: string[]) {
    try {
      const added: { path: string; name: string }[] = await invoke('add_paths', { paths });
      addFiles(added);
    } catch (e) {
      summary = { ok: 0, failed: [{ name: '(drop)', error: String(e) }] };
    }
  }

  async function processPhotos() {
    if (app.files.length === 0) return;
    const folder = await open({ directory: true, multiple: false });
    if (typeof folder !== 'string') return; // cancelled

    busy = true;
    summary = null;
    let ok = 0;
    const failed: { name: string; error: string }[] = [];

    for (const f of app.files) {
      const metadata = app.mode === 'shared' ? app.shared : f.metadata;
      try {
        await invoke('process_one', {
          args: {
            src: f.path,
            output_dir: folder,
            metadata
          }
        });
        ok += 1;
      } catch (e) {
        failed.push({ name: f.name, error: String(e) });
      }
    }
    busy = false;
    summary = { ok, failed };
  }

  let canProcess = $derived(app.files.length > 0 && !busy);
</script>

<div class="app">
  <header>
    <div class="brand">
      <span class="dot" aria-hidden="true"></span>
      <h1>MetaTag Studio</h1>
    </div>
    <p class="tagline">Add SEO + accessibility metadata to listing photos.</p>
  </header>

  <main>
    <section class="pane left">
      <FileList {dragOver} />
    </section>

    <section class="pane right">
      <div class="toolbar">
        <ModeToggle />
      </div>
      <div class="editor-wrap">
        <MetadataEditor />
      </div>
    </section>
  </main>

  <footer>
    {#if summary}
      <div class="summary" class:has-fail={summary.failed.length > 0}>
        <strong
          >Done: {summary.ok} succeeded, {summary.failed.length} failed</strong
        >
        {#if summary.failed.length > 0}
          <ul>
            {#each summary.failed as f}
              <li><span class="fail-name">{f.name}</span>: {f.error}</li>
            {/each}
          </ul>
        {/if}
      </div>
    {/if}
    <div class="actions">
      <button class="primary" disabled={!canProcess} onclick={processPhotos}>
        {busy ? 'Processing…' : 'Process Photos'}
      </button>
    </div>
  </footer>
</div>

<style>
  :global(html, body) {
    height: 100%;
    margin: 0;
    overflow: hidden;
  }

  :global(:root) {
    font-family: 'Segoe UI', Inter, system-ui, -apple-system, sans-serif;
    font-size: 14px;
    color: #1a1a1a;
    background: #f5f6f9;
  }

  .app {
    display: grid;
    grid-template-rows: auto 1fr auto;
    height: 100vh;
    background: #f5f6f9;
  }

  header {
    padding: 0.9rem 1.5rem 0.6rem;
    background: #fff;
    border-bottom: 1px solid #ebedf2;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.55rem;
  }

  .brand .dot {
    width: 14px;
    height: 14px;
    border-radius: 4px;
    background: linear-gradient(135deg, #2b5fd9, #1a3da3);
  }

  header h1 {
    margin: 0;
    font-size: 1.15rem;
    font-weight: 600;
  }

  .tagline {
    margin: 0.15rem 0 0 1.6rem;
    color: #6c707a;
    font-size: 0.85rem;
  }

  main {
    display: grid;
    grid-template-columns: minmax(280px, 360px) 1fr;
    gap: 1.25rem;
    padding: 1.25rem 1.5rem;
    overflow: hidden;
    min-height: 0;
  }

  .pane {
    background: #fff;
    border-radius: 14px;
    border: 1px solid #ebedf2;
    padding: 1rem;
    box-shadow: 0 1px 2px rgba(20, 30, 70, 0.03);
    overflow: hidden;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .pane.right {
    gap: 1rem;
  }

  .toolbar {
    display: flex;
    justify-content: flex-end;
  }

  .editor-wrap {
    flex: 1;
    overflow-y: auto;
    padding-right: 0.25rem;
  }

  footer {
    padding: 0.85rem 1.5rem 1.1rem;
    background: #fff;
    border-top: 1px solid #ebedf2;
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
  }


  .primary {
    font: inherit;
    font-size: 0.95rem;
    font-weight: 600;
    padding: 0.7rem 1.4rem;
    border-radius: 9px;
    border: none;
    background: #2b5fd9;
    color: #fff;
    cursor: pointer;
    box-shadow: 0 1px 2px rgba(43, 95, 217, 0.25);
    transition: background 0.15s, transform 0.05s, box-shadow 0.15s;
  }

  .primary:hover:not(:disabled) {
    background: #224dbd;
    box-shadow: 0 2px 6px rgba(43, 95, 217, 0.35);
  }

  .primary:active:not(:disabled) {
    transform: translateY(1px);
  }

  .primary:disabled {
    background: #c4ccdf;
    cursor: not-allowed;
    box-shadow: none;
  }

  .summary {
    background: #f0f7f0;
    border: 1px solid #cee0ce;
    color: #2c6b3f;
    padding: 0.6rem 0.85rem;
    border-radius: 9px;
    font-size: 0.9rem;
  }

  .summary.has-fail {
    background: #fcf2f0;
    border-color: #ecccc7;
    color: #8a3030;
  }

  .summary ul {
    margin: 0.4rem 0 0;
    padding-left: 1.1rem;
  }

  .summary li {
    margin: 0.15rem 0;
  }

  .fail-name {
    font-weight: 600;
  }
</style>
